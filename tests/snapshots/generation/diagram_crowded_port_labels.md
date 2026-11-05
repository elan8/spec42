# META
~~~ini
description=Ten named connectors ending at neighbouring ports on one controller reserve measured port-label boxes and keep crowded connector names in tooltips when no clear label position exists
type=generate
libraries=standard
require_complete_publication=true
require_no_diagnostics=true
plugin=native:diagram
viewKind=interconnection-view
viewDocument=model.sysml
viewQualifiedName=CrowdedPorts::topology
~~~
# SOURCE
## model.sysml
~~~sysml
package CrowdedPorts {
    private import StandardViewDefinitions::*;
    port def Signal { out attribute value : ScalarValues::Boolean; }
    part def Endpoint { port data : Signal; }
    part def Controller {
        port channel01 : ~Signal;
        port channel02 : ~Signal;
        port channel03 : ~Signal;
        port channel04 : ~Signal;
        port channel05 : ~Signal;
        port channel06 : ~Signal;
        port channel07 : ~Signal;
        port channel08 : ~Signal;
        port channel09 : ~Signal;
        port channel10 : ~Signal;
    }
    part network {
        part controller : Controller;
        part endpoint01 : Endpoint;
        part endpoint02 : Endpoint;
        part endpoint03 : Endpoint;
        part endpoint04 : Endpoint;
        part endpoint05 : Endpoint;
        part endpoint06 : Endpoint;
        part endpoint07 : Endpoint;
        part endpoint08 : Endpoint;
        part endpoint09 : Endpoint;
        part endpoint10 : Endpoint;
        connection link01 connect endpoint01.data to controller.channel01;
        connection link02 connect endpoint02.data to controller.channel02;
        connection link03 connect endpoint03.data to controller.channel03;
        connection link04 connect endpoint04.data to controller.channel04;
        connection link05 connect endpoint05.data to controller.channel05;
        connection link06 connect endpoint06.data to controller.channel06;
        connection link07 connect endpoint07.data to controller.channel07;
        connection link08 connect endpoint08.data to controller.channel08;
        connection link09 connect endpoint09.data to controller.channel09;
        connection link10 connect endpoint10.data to controller.channel10;
    }
    view topology : InterconnectionView { expose network; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:5389f4128d554a23f429b13216a6e7d32b9e3569ed0bbf812c15ca868f6f69ec") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Signal") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal::value"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Boolean")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Controller")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint01"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Endpoint")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint02"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Endpoint")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint03"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Endpoint")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint04"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Endpoint")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint05"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Endpoint")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint06"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Endpoint")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint07"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Endpoint")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint08"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Endpoint")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint09"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Endpoint")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint10"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Endpoint")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "endpoint01::data")) (memberAccessOperand (reference "controller::channel01")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "endpoint02::data")) (memberAccessOperand (reference "controller::channel02")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "endpoint03::data")) (memberAccessOperand (reference "controller::channel03")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "endpoint04::data")) (memberAccessOperand (reference "controller::channel04")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "endpoint05::data")) (memberAccessOperand (reference "controller::channel05")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "endpoint06::data")) (memberAccessOperand (reference "controller::channel06")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "endpoint07::data")) (memberAccessOperand (reference "controller::channel07")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "endpoint08::data")) (memberAccessOperand (reference "controller::channel08")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "endpoint09::data")) (memberAccessOperand (reference "controller::channel09")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "endpoint10::data")) (memberAccessOperand (reference "controller::channel10")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::topology"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind view) (name "topology")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "network")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal::value"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (kind featureTyping) (ordinal 0))
      (authored-target "Controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint01"))) (kind featureTyping) (ordinal 0))
      (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint02"))) (kind featureTyping) (ordinal 0))
      (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint03"))) (kind featureTyping) (ordinal 0))
      (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint04"))) (kind featureTyping) (ordinal 0))
      (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint05"))) (kind featureTyping) (ordinal 0))
      (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint06"))) (kind featureTyping) (ordinal 0))
      (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint07"))) (kind featureTyping) (ordinal 0))
      (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint08"))) (kind featureTyping) (ordinal 0))
      (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint09"))) (kind featureTyping) (ordinal 0))
      (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint10"))) (kind featureTyping) (ordinal 0))
      (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "endpoint01::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "controller::channel01")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "endpoint02::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "controller::channel02")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "endpoint03::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "controller::channel03")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "endpoint04::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "controller::channel04")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "endpoint05::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "controller::channel05")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "endpoint06::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "controller::channel06")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "endpoint07::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "controller::channel07")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "endpoint08::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "controller::channel08")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "endpoint09::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "controller::channel09")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "endpoint10::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "controller::channel10")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::topology"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind view) (name "topology")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "network")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))))
  )
  (relationships
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal::value"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal::value"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint01"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint01"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint02"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint02"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint03"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint03"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint04"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint04"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint05"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint05"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint06"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint06"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint07"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint07"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint08"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint08"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint09"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint09"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint10"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint10"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::topology"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::topology"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind view) (name "topology")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind view) (name "topology")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal::value"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal::value"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint01"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint01"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint01"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint02"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint02"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint02"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint03"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint03"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint03"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint04"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint04"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint04"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint05"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint05"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint05"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint06"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint06"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint06"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint07"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint07"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint07"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint08"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint08"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint08"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint09"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint09"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint09"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint10"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint10"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint10"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::topology"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind view) (name "topology")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::topology"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint01")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint02")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint03")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint04")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint05")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint06")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint07")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint08")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint09")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint10")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal::value")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
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
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint01")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint02")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint03")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint04")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint05")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint06")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint07")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint08")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint09")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint10")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::topology")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind view) (name "topology")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::topology")))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint01"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint01")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) "endpoint01::data")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01"))) "controller::channel01")))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint02"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint02")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) "endpoint02::data")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02"))) "controller::channel02")))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint03"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint03")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) "endpoint03::data")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03"))) "controller::channel03")))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint04"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint04")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) "endpoint04::data")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04"))) "controller::channel04")))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint05"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint05")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) "endpoint05::data")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05"))) "controller::channel05")))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint06"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint06")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) "endpoint06::data")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06"))) "controller::channel06")))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint07"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint07")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) "endpoint07::data")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07"))) "controller::channel07")))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint08"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint08")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) "endpoint08::data")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08"))) "controller::channel08")))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint09"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint09")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) "endpoint09::data")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09"))) "controller::channel09")))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint10"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint10")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) "endpoint10::data")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller")) (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10"))) "controller::channel10")))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 26) (end 5 32)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 26) (end 6 32)) (probe (position 6 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 26) (end 7 32)) (probe (position 7 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 26) (end 8 32)) (probe (position 8 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 26) (end 9 32)) (probe (position 9 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 26) (end 10 32)) (probe (position 10 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 26) (end 11 32)) (probe (position 11 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 26) (end 12 32)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 26) (end 13 32)) (probe (position 13 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 26) (end 14 32)) (probe (position 14 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 36) (end 3 42)) (probe (position 3 36))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 2 44) (end 2 65)) (probe (position 2 44))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Signal::value"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 26) (end 17 36)) (probe (position 17 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::controller"))) (kind featureTyping) (ordinal 0) (authored-target "Controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 26) (end 18 34)) (probe (position 18 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint01"))) (kind featureTyping) (ordinal 0) (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 19 26) (end 19 34)) (probe (position 19 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint02"))) (kind featureTyping) (ordinal 0) (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 20 26) (end 20 34)) (probe (position 20 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint03"))) (kind featureTyping) (ordinal 0) (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 21 26) (end 21 34)) (probe (position 21 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint04"))) (kind featureTyping) (ordinal 0) (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 22 26) (end 22 34)) (probe (position 22 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint05"))) (kind featureTyping) (ordinal 0) (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 23 26) (end 23 34)) (probe (position 23 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint06"))) (kind featureTyping) (ordinal 0) (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 24 26) (end 24 34)) (probe (position 24 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint07"))) (kind featureTyping) (ordinal 0) (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 25 26) (end 25 34)) (probe (position 25 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint08"))) (kind featureTyping) (ordinal 0) (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 26 26) (end 26 34)) (probe (position 26 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint09"))) (kind featureTyping) (ordinal 0) (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 27 26) (end 27 34)) (probe (position 27 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::endpoint10"))) (kind featureTyping) (ordinal 0) (authored-target "Endpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 28 34) (end 28 49)) (probe (position 28 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (kind memberAccessOperand) (ordinal 0) (authored-target "endpoint01::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 28 53) (end 28 73)) (probe (position 28 53))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link01"))) (kind memberAccessOperand) (ordinal 1) (authored-target "controller::channel01")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel01")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 29 34) (end 29 49)) (probe (position 29 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (kind memberAccessOperand) (ordinal 0) (authored-target "endpoint02::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 29 53) (end 29 73)) (probe (position 29 53))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link02"))) (kind memberAccessOperand) (ordinal 1) (authored-target "controller::channel02")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel02")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 30 34) (end 30 49)) (probe (position 30 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (kind memberAccessOperand) (ordinal 0) (authored-target "endpoint03::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 30 53) (end 30 73)) (probe (position 30 53))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link03"))) (kind memberAccessOperand) (ordinal 1) (authored-target "controller::channel03")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel03")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 31 34) (end 31 49)) (probe (position 31 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (kind memberAccessOperand) (ordinal 0) (authored-target "endpoint04::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 31 53) (end 31 73)) (probe (position 31 53))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link04"))) (kind memberAccessOperand) (ordinal 1) (authored-target "controller::channel04")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel04")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 32 34) (end 32 49)) (probe (position 32 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (kind memberAccessOperand) (ordinal 0) (authored-target "endpoint05::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 32 53) (end 32 73)) (probe (position 32 53))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link05"))) (kind memberAccessOperand) (ordinal 1) (authored-target "controller::channel05")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel05")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 33 34) (end 33 49)) (probe (position 33 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (kind memberAccessOperand) (ordinal 0) (authored-target "endpoint06::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 33 53) (end 33 73)) (probe (position 33 53))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link06"))) (kind memberAccessOperand) (ordinal 1) (authored-target "controller::channel06")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel06")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 34 34) (end 34 49)) (probe (position 34 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (kind memberAccessOperand) (ordinal 0) (authored-target "endpoint07::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 34 53) (end 34 73)) (probe (position 34 53))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link07"))) (kind memberAccessOperand) (ordinal 1) (authored-target "controller::channel07")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel07")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 35 34) (end 35 49)) (probe (position 35 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (kind memberAccessOperand) (ordinal 0) (authored-target "endpoint08::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 35 53) (end 35 73)) (probe (position 35 53))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link08"))) (kind memberAccessOperand) (ordinal 1) (authored-target "controller::channel08")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel08")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 36 34) (end 36 49)) (probe (position 36 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (kind memberAccessOperand) (ordinal 0) (authored-target "endpoint09::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 36 53) (end 36 73)) (probe (position 36 53))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link09"))) (kind memberAccessOperand) (ordinal 1) (authored-target "controller::channel09")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel09")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 37 34) (end 37 49)) (probe (position 37 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (kind memberAccessOperand) (ordinal 0) (authored-target "endpoint10::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Endpoint::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 37 53) (end 37 73)) (probe (position 37 53))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network::link10"))) (kind memberAccessOperand) (ordinal 1) (authored-target "controller::channel10")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::Controller::channel10")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 39 20) (end 39 39)) (probe (position 39 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::topology"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 39 49) (end 39 56)) (probe (position 39 49))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "CrowdedPorts")) (named (kind view) (name "topology")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "network")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "CrowdedPorts::network")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 12,
  "modelDigest": "blake3:d43bff0f0e8c72aa2bfef2aa6c328095f196a02e8a6e297b777e802c6b8a55c2",
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
        3,
        29,
        3,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        36,
        3,
        42
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        13,
        5,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        26,
        5,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        13,
        6,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        26,
        6,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        13,
        7,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        26,
        7,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        13,
        8,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        26,
        8,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        13,
        9,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        26,
        9,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        13,
        10,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        26,
        10,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        13,
        11,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        26,
        11,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        13,
        12,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        26,
        12,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        13,
        13,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        26,
        13,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        13,
        14,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        26,
        14,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        9,
        16,
        16
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        13,
        17,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        26,
        17,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        13,
        18,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        26,
        18,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        13,
        19,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        26,
        19,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        20,
        13,
        20,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        20,
        26,
        20,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        13,
        21,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        26,
        21,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        13,
        22,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        26,
        22,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        23,
        13,
        23,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        23,
        26,
        23,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        13,
        24,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        26,
        24,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        25,
        13,
        25,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        25,
        26,
        25,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        26,
        13,
        26,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        26,
        26,
        26,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        27,
        13,
        27,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        27,
        26,
        27,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        28,
        19,
        28,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        28,
        34,
        28,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        28,
        53,
        28,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        29,
        19,
        29,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        29,
        34,
        29,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        29,
        53,
        29,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        30,
        19,
        30,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        30,
        34,
        30,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        30,
        53,
        30,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        31,
        19,
        31,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        31,
        34,
        31,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        31,
        53,
        31,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        32,
        19,
        32,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        32,
        34,
        32,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        32,
        53,
        32,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        33,
        19,
        33,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        33,
        34,
        33,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        33,
        53,
        33,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        34,
        19,
        34,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        34,
        34,
        34,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        34,
        53,
        34,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        35,
        19,
        35,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        35,
        34,
        35,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        35,
        53,
        35,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        36,
        19,
        36,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        36,
        34,
        36,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        36,
        53,
        36,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        37,
        19,
        37,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        37,
        34,
        37,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        37,
        53,
        37,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        39,
        9,
        39,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Controller"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Controller::channel01"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Controller::channel02"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Controller::channel03"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Controller::channel04"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Controller::channel05"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Controller::channel06"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Controller::channel07"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Controller::channel08"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Controller::channel09"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Controller::channel10"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Endpoint"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Endpoint::data"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::Signal"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::controller"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::endpoint01"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::endpoint02"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::endpoint03"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::endpoint04"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::endpoint05"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::endpoint06"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::endpoint07"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::endpoint08"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::endpoint09"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::endpoint10"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::link01"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::link02"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::link03"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::link04"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::link05"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::link06"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::link07"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::link08"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::link09"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::network::link10"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "CrowdedPorts::topology"
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "typing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "typing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "typing",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "typing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "connection",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "connection",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "connection",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "connection",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "connection",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "connection",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 44,
      "relationshipKind": "connection",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "connection",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "connection",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "connection",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 67,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 75,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 83,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 91,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 99,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 107,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 115,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 123,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 131,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "noncomposite-membership",
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
      "ordinal": 12,
      "relationshipKind": "noncomposite-membership",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 55,
      "relationshipKind": "typing",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "noncomposite-membership",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 63,
      "relationshipKind": "typing",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "noncomposite-membership",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 71,
      "relationshipKind": "typing",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "noncomposite-membership",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 79,
      "relationshipKind": "typing",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "noncomposite-membership",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 87,
      "relationshipKind": "typing",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "noncomposite-membership",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 95,
      "relationshipKind": "typing",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "noncomposite-membership",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 103,
      "relationshipKind": "typing",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "noncomposite-membership",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 111,
      "relationshipKind": "typing",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "noncomposite-membership",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 119,
      "relationshipKind": "typing",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "noncomposite-membership",
      "source": 25
    },
    {
      "kind": "relationship",
      "ordinal": 127,
      "relationshipKind": "typing",
      "source": 25
    },
    {
      "kind": "relationship",
      "ordinal": 135,
      "relationshipKind": "memberAccessOperand",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 136,
      "relationshipKind": "memberAccessOperand",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 141,
      "relationshipKind": "memberAccessOperand",
      "source": 27
    },
    {
      "kind": "relationship",
      "ordinal": 142,
      "relationshipKind": "memberAccessOperand",
      "source": 27
    },
    {
      "kind": "relationship",
      "ordinal": 147,
      "relationshipKind": "memberAccessOperand",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 148,
      "relationshipKind": "memberAccessOperand",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 153,
      "relationshipKind": "memberAccessOperand",
      "source": 29
    },
    {
      "kind": "relationship",
      "ordinal": 154,
      "relationshipKind": "memberAccessOperand",
      "source": 29
    },
    {
      "kind": "relationship",
      "ordinal": 159,
      "relationshipKind": "memberAccessOperand",
      "source": 30
    },
    {
      "kind": "relationship",
      "ordinal": 160,
      "relationshipKind": "memberAccessOperand",
      "source": 30
    },
    {
      "kind": "relationship",
      "ordinal": 165,
      "relationshipKind": "memberAccessOperand",
      "source": 31
    },
    {
      "kind": "relationship",
      "ordinal": 166,
      "relationshipKind": "memberAccessOperand",
      "source": 31
    },
    {
      "kind": "relationship",
      "ordinal": 171,
      "relationshipKind": "memberAccessOperand",
      "source": 32
    },
    {
      "kind": "relationship",
      "ordinal": 172,
      "relationshipKind": "memberAccessOperand",
      "source": 32
    },
    {
      "kind": "relationship",
      "ordinal": 177,
      "relationshipKind": "memberAccessOperand",
      "source": 33
    },
    {
      "kind": "relationship",
      "ordinal": 178,
      "relationshipKind": "memberAccessOperand",
      "source": 33
    },
    {
      "kind": "relationship",
      "ordinal": 183,
      "relationshipKind": "memberAccessOperand",
      "source": 34
    },
    {
      "kind": "relationship",
      "ordinal": 184,
      "relationshipKind": "memberAccessOperand",
      "source": 34
    },
    {
      "kind": "relationship",
      "ordinal": 189,
      "relationshipKind": "memberAccessOperand",
      "source": 35
    },
    {
      "kind": "relationship",
      "ordinal": 190,
      "relationshipKind": "memberAccessOperand",
      "source": 35
    }
  ],
  "selectedView": {
    "reference": 36,
    "kind": "interconnection-view",
    "name": "topology",
    "source": 75
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 23,
        "origin": 11,
        "provenance": "authored",
        "reference": 67,
        "source": 0,
        "target": 11
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 2,
        "origin": 12,
        "provenance": "implied",
        "reference": 88,
        "source": 11,
        "target": 12
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 4,
        "origin": 13,
        "provenance": "implied",
        "reference": 89,
        "source": 11,
        "target": 13
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 6,
        "origin": 14,
        "provenance": "implied",
        "reference": 90,
        "source": 11,
        "target": 14
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 8,
        "origin": 15,
        "provenance": "implied",
        "reference": 91,
        "source": 11,
        "target": 15
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 10,
        "origin": 16,
        "provenance": "implied",
        "reference": 92,
        "source": 11,
        "target": 16
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 12,
        "origin": 17,
        "provenance": "implied",
        "reference": 93,
        "source": 11,
        "target": 17
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 14,
        "origin": 18,
        "provenance": "implied",
        "reference": 94,
        "source": 11,
        "target": 18
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 16,
        "origin": 19,
        "provenance": "implied",
        "reference": 95,
        "source": 11,
        "target": 19
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 18,
        "origin": 20,
        "provenance": "implied",
        "reference": 96,
        "source": 11,
        "target": 20
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 20,
        "origin": 21,
        "provenance": "implied",
        "reference": 97,
        "source": 11,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 25,
        "origin": 22,
        "provenance": "authored",
        "reference": 68,
        "source": 0,
        "target": 22
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 23,
        "provenance": "implied",
        "reference": 99,
        "source": 22,
        "target": 23
      },
      {
        "kind": "containment",
        "navigation": 27,
        "origin": 24,
        "provenance": "authored",
        "reference": 69,
        "source": 0,
        "target": 24
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 25,
        "provenance": "implied",
        "reference": 101,
        "source": 24,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 29,
        "origin": 26,
        "provenance": "authored",
        "reference": 70,
        "source": 0,
        "target": 26
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 27,
        "provenance": "implied",
        "reference": 103,
        "source": 26,
        "target": 27
      },
      {
        "kind": "containment",
        "navigation": 31,
        "origin": 28,
        "provenance": "authored",
        "reference": 71,
        "source": 0,
        "target": 28
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 29,
        "provenance": "implied",
        "reference": 105,
        "source": 28,
        "target": 29
      },
      {
        "kind": "containment",
        "navigation": 33,
        "origin": 30,
        "provenance": "authored",
        "reference": 72,
        "source": 0,
        "target": 30
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 31,
        "provenance": "implied",
        "reference": 107,
        "source": 30,
        "target": 31
      },
      {
        "kind": "containment",
        "navigation": 35,
        "origin": 32,
        "provenance": "authored",
        "reference": 73,
        "source": 0,
        "target": 32
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 33,
        "provenance": "implied",
        "reference": 109,
        "source": 32,
        "target": 33
      },
      {
        "kind": "containment",
        "navigation": 37,
        "origin": 34,
        "provenance": "authored",
        "reference": 74,
        "source": 0,
        "target": 34
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 35,
        "provenance": "implied",
        "reference": 111,
        "source": 34,
        "target": 35
      },
      {
        "kind": "containment",
        "navigation": 39,
        "origin": 36,
        "provenance": "authored",
        "reference": 75,
        "source": 0,
        "target": 36
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 37,
        "provenance": "implied",
        "reference": 113,
        "source": 36,
        "target": 37
      },
      {
        "kind": "containment",
        "navigation": 41,
        "origin": 38,
        "provenance": "authored",
        "reference": 76,
        "source": 0,
        "target": 38
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 39,
        "provenance": "implied",
        "reference": 115,
        "source": 38,
        "target": 39
      },
      {
        "kind": "containment",
        "navigation": 43,
        "origin": 40,
        "provenance": "authored",
        "reference": 77,
        "source": 0,
        "target": 40
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 41,
        "provenance": "implied",
        "reference": 117,
        "source": 40,
        "target": 41
      },
      {
        "kind": "containment",
        "navigation": 45,
        "origin": 1,
        "provenance": "authored",
        "reference": 78,
        "source": 0,
        "target": 1
      },
      {
        "kind": "connector",
        "navigation": 46,
        "origin": 1,
        "provenance": "authored",
        "reference": 47,
        "source": 23,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 48,
        "origin": 2,
        "provenance": "authored",
        "reference": 79,
        "source": 0,
        "target": 2
      },
      {
        "kind": "connector",
        "navigation": 49,
        "origin": 2,
        "provenance": "authored",
        "reference": 48,
        "source": 25,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 51,
        "origin": 3,
        "provenance": "authored",
        "reference": 80,
        "source": 0,
        "target": 3
      },
      {
        "kind": "connector",
        "navigation": 52,
        "origin": 3,
        "provenance": "authored",
        "reference": 49,
        "source": 27,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 54,
        "origin": 4,
        "provenance": "authored",
        "reference": 81,
        "source": 0,
        "target": 4
      },
      {
        "kind": "connector",
        "navigation": 55,
        "origin": 4,
        "provenance": "authored",
        "reference": 50,
        "source": 29,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 57,
        "origin": 5,
        "provenance": "authored",
        "reference": 82,
        "source": 0,
        "target": 5
      },
      {
        "kind": "connector",
        "navigation": 58,
        "origin": 5,
        "provenance": "authored",
        "reference": 51,
        "source": 31,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 60,
        "origin": 6,
        "provenance": "authored",
        "reference": 83,
        "source": 0,
        "target": 6
      },
      {
        "kind": "connector",
        "navigation": 61,
        "origin": 6,
        "provenance": "authored",
        "reference": 52,
        "source": 33,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 63,
        "origin": 7,
        "provenance": "authored",
        "reference": 84,
        "source": 0,
        "target": 7
      },
      {
        "kind": "connector",
        "navigation": 64,
        "origin": 7,
        "provenance": "authored",
        "reference": 53,
        "source": 35,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 66,
        "origin": 8,
        "provenance": "authored",
        "reference": 85,
        "source": 0,
        "target": 8
      },
      {
        "kind": "connector",
        "navigation": 67,
        "origin": 8,
        "provenance": "authored",
        "reference": 54,
        "source": 37,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 69,
        "origin": 9,
        "provenance": "authored",
        "reference": 86,
        "source": 0,
        "target": 9
      },
      {
        "kind": "connector",
        "navigation": 70,
        "origin": 9,
        "provenance": "authored",
        "reference": 55,
        "source": 39,
        "target": 20
      },
      {
        "kind": "containment",
        "navigation": 72,
        "origin": 10,
        "provenance": "authored",
        "reference": 87,
        "source": 0,
        "target": 10
      },
      {
        "kind": "connector",
        "navigation": 73,
        "origin": 10,
        "provenance": "authored",
        "reference": 56,
        "source": 41,
        "target": 21
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
        8,
        9,
        10
      ],
      "parts": [
        0,
        11,
        22,
        24,
        26,
        28,
        30,
        32,
        34,
        36,
        38,
        40
      ],
      "portFamilies": [
        {
          "connectors": [
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10
          ],
          "general": [
            {
              "label": "Signal",
              "reference": 13
            }
          ],
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      ],
      "ports": [
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
        23,
        25,
        27,
        29,
        31,
        33,
        35,
        37,
        39,
        41
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              11,
              22,
              24,
              26,
              28,
              30,
              32,
              34,
              36,
              38,
              40
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
              8,
              9,
              10
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "network",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:7f461b63828bc0c3aab8bd33abfc67014b3218a416772490e09760b840d9ecd9",
        "reference": 14,
        "source": 22,
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
        "name": "link01",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:75d2e14eb2e3e7c9d803d53ecf7a41708b508c900f3b5218246e56bbb27fdf70",
        "reference": 26,
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
        "name": "link02",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:1683ad169f77049b8bd2402d159e5764d382413caf8f4c7549fdb4346e97ce0a",
        "reference": 27,
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
        "name": "link03",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:d8c5e7f69a1be9ee10076f9b531a40b0bef20febad68b54cf300704502c17698",
        "reference": 28,
        "source": 51,
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
        "name": "link04",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:e08ee9f3b7f3e4d499d8e51c1452c17febb8187b6e11277d8cfd41d8236a8955",
        "reference": 29,
        "source": 54,
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
        "name": "link05",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:b77a14b6b72e49aecf25da530af5edf476bc8c69f233e81a42f8d3dab5f5b5bf",
        "reference": 30,
        "source": 57,
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
        "name": "link06",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:8b6d579fac6e0f4f15f4c48219b6549ff3b367288747962d1f8a9480fc4a9982",
        "reference": 31,
        "source": 60,
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
        "name": "link07",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:a75b6babdb3aeaed0b9ab16efe08c59ad78a809f77379d99acf5540bfec21f34",
        "reference": 32,
        "source": 63,
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
        "name": "link08",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:187044ce019c8fefdc3a90d7b9925c5afdc5c071bdb1d4aa4c15e650096b8a16",
        "reference": 33,
        "source": 66,
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
        "name": "link09",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:1a4a425c7d3afff05a51ea3e423679f00ecca04f15b5d599109f49d94af8d26e",
        "reference": 34,
        "source": 69,
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
        "name": "link10",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:d2938f11694e028f855b8474e488bab07c7bc853bd0b48912c5b92648dfa6e12",
        "reference": 35,
        "source": 72,
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
              12,
              13,
              14,
              15,
              16,
              17,
              18,
              19,
              20,
              21
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "controller",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:e92e300b013a9e9eabd663b3ceb7ca0558a796393bda833a984b135ff5bc2f85",
        "reference": 15,
        "source": 23,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Controller",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": "in",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "channel01",
        "notationRole": "usage",
        "owner": 11,
        "presentationKey": "p:e58ea513842cc51f20d6548ec7a04f40e79c24744b435b72494af9b2250a619f",
        "reference": 1,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": "in",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "channel02",
        "notationRole": "usage",
        "owner": 11,
        "presentationKey": "p:5213f0cad72e7178e89363372f864213f8a5284a926ef6ceea23d041c649e719",
        "reference": 2,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": "in",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "channel03",
        "notationRole": "usage",
        "owner": 11,
        "presentationKey": "p:97c456938c096c9f8b51d93c69a9e8ee1ed8fdd79a211df96ffc12a36b0bc8d6",
        "reference": 3,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": "in",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "channel04",
        "notationRole": "usage",
        "owner": 11,
        "presentationKey": "p:b9a7261cbc253f996867a8fd80fb4387ff7a867cc9ea7f766f4645e37ffdfce6",
        "reference": 4,
        "source": 8,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": "in",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "channel05",
        "notationRole": "usage",
        "owner": 11,
        "presentationKey": "p:ab5d7a52f05bf47f889a532e12ad5ff0223111a913c6f44d5b3e5d70ead4eac0",
        "reference": 5,
        "source": 10,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": "in",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "channel06",
        "notationRole": "usage",
        "owner": 11,
        "presentationKey": "p:34db5d4af3bbd608948b2ef431e38a1c98310e419b244c2285da0529b5c61695",
        "reference": 6,
        "source": 12,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": "in",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "channel07",
        "notationRole": "usage",
        "owner": 11,
        "presentationKey": "p:c9d46a2559d676360aa387f0399ab7738e142b48b0109eb463318e9417333097",
        "reference": 7,
        "source": 14,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": "in",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "channel08",
        "notationRole": "usage",
        "owner": 11,
        "presentationKey": "p:bfc932b8611968f687e6819e26961cb489c44ca51834d41b59e82c6aaee2fa98",
        "reference": 8,
        "source": 16,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": "in",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "channel09",
        "notationRole": "usage",
        "owner": 11,
        "presentationKey": "p:b069724b5366ed2b0140eb77c3daaaf7afbdfa29d7cb935038cc3e3b11d55434",
        "reference": 9,
        "source": 18,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": "in",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "channel10",
        "notationRole": "usage",
        "owner": 11,
        "presentationKey": "p:ab36802f30dfd4f2c4989d81eca7b7518630241e76789951522b4eb70b750876",
        "reference": 10,
        "source": 20,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              23
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "endpoint01",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:6ca509a99cb04f916103aa3b199fe4e50b5b2d340a979d635152b0cfb29615a4",
        "reference": 16,
        "source": 25,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Endpoint",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "data",
        "notationRole": "usage",
        "owner": 22,
        "presentationKey": "p:e42d1d962f81919a810e84489e899db3b3f5e6ae3e2ef0692e1a43f68def60fe",
        "reference": 12,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
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
        "name": "endpoint02",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:8132af2df76546d3c255298fa14971bbd8103ac3c193a6c1ddb109692fe75c30",
        "reference": 17,
        "source": 27,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Endpoint",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "data",
        "notationRole": "usage",
        "owner": 24,
        "presentationKey": "p:0bc23202996f8f1dee40b3289b25b4cfa429e4a17c7a7030c668405d325543bd",
        "reference": 12,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              27
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "endpoint03",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:eda5fc7798bcbf76a7a4120f0678be8e94d8d7d23ee58f9bf09152eb828a0454",
        "reference": 18,
        "source": 29,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Endpoint",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "data",
        "notationRole": "usage",
        "owner": 26,
        "presentationKey": "p:b9bdbe30fea384cfddd677ede6c89331cb659f0bdca545d0c51a7035c3f17f77",
        "reference": 12,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              29
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "endpoint04",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:5d7a38ad392a805ef37d0b7e2a11b2eb612496abac671a8297deefb9de9c6136",
        "reference": 19,
        "source": 31,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Endpoint",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "data",
        "notationRole": "usage",
        "owner": 28,
        "presentationKey": "p:49d392fb22ed75237a3923d22b7d5a88e57e496b8d20e912d32263b885ecff19",
        "reference": 12,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              31
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "endpoint05",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:a6b94696976b3b0d28b5802ae016b70f86a418da790189cc657b5f6d1621b004",
        "reference": 20,
        "source": 33,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Endpoint",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "data",
        "notationRole": "usage",
        "owner": 30,
        "presentationKey": "p:f26e633429470f8260bd68b080f8e4e192c5d47f5d2b7c6b1f4dd56ca20c820d",
        "reference": 12,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              33
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "endpoint06",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:741bc240bb157a2f993edd22abc2009c7ef4f54cb4f39ba2f344a762780afb3f",
        "reference": 21,
        "source": 35,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Endpoint",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "data",
        "notationRole": "usage",
        "owner": 32,
        "presentationKey": "p:1560314feea5ac2ae6bf6f20b3427d557c7ba511f86f9320a3291ae9238ec17c",
        "reference": 12,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              35
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "endpoint07",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:11945f23bdb5e59c523fdf6e706a55a9c4aaa0df65bfee2567d1fc097f5b9f17",
        "reference": 22,
        "source": 37,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Endpoint",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "data",
        "notationRole": "usage",
        "owner": 34,
        "presentationKey": "p:92dcc86e33279303e0c6421673cb8379f3dc15fc7e74f56ae4e58f360a8bf0a3",
        "reference": 12,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              37
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "endpoint08",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:391b7ce7ddd766ca2ba60deb97101d68cb1a21f841a3e315c65ac9e88dd87574",
        "reference": 23,
        "source": 39,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Endpoint",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "data",
        "notationRole": "usage",
        "owner": 36,
        "presentationKey": "p:adaf3e0a847959ed4a11087f2cb0607f1169c0528e4008c5c1c4953d874d902c",
        "reference": 12,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              39
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "endpoint09",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:fc00d2c1c54f4d1565f8a174596509cfbd2ff248f3e85fded00b5ecf136fb4b3",
        "reference": 24,
        "source": 41,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Endpoint",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "data",
        "notationRole": "usage",
        "owner": 38,
        "presentationKey": "p:ead0f7a5e0e692dd518ea0162e626ac1501ff05915ac434066e54a6c5cc4e9b2",
        "reference": 12,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              41
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "endpoint10",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:bfb3debeea4df149724294941103ff0649f97f6c00ccaacde1de749d75fad290",
        "reference": 25,
        "source": 43,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Endpoint",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "data",
        "notationRole": "usage",
        "owner": 40,
        "presentationKey": "p:944e192b59e48de06d8b086ed057fe5a3ed9d8d2c3c6f9cf5192a41ef5c8c71f",
        "reference": 12,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 13
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 24,
        "provenance": "authored",
        "reference": 98,
        "source": 11,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 37,
        "source": 12,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 38,
        "source": 13,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 7,
        "provenance": "authored",
        "reference": 39,
        "source": 14,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 40,
        "source": 15,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 41,
        "source": 16,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 13,
        "provenance": "authored",
        "reference": 42,
        "source": 17,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 15,
        "provenance": "authored",
        "reference": 43,
        "source": 18,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 17,
        "provenance": "authored",
        "reference": 44,
        "source": 19,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 19,
        "provenance": "authored",
        "reference": 45,
        "source": 20,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 21,
        "provenance": "authored",
        "reference": 46,
        "source": 21,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 26,
        "provenance": "authored",
        "reference": 100,
        "source": 22,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 57,
        "source": 23,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 28,
        "provenance": "authored",
        "reference": 102,
        "source": 24,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 58,
        "source": 25,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 30,
        "provenance": "authored",
        "reference": 104,
        "source": 26,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 59,
        "source": 27,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 32,
        "provenance": "authored",
        "reference": 106,
        "source": 28,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 60,
        "source": 29,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 34,
        "provenance": "authored",
        "reference": 108,
        "source": 30,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 61,
        "source": 31,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 36,
        "provenance": "authored",
        "reference": 110,
        "source": 32,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 62,
        "source": 33,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 38,
        "provenance": "authored",
        "reference": 112,
        "source": 34,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 63,
        "source": 35,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 40,
        "provenance": "authored",
        "reference": 114,
        "source": 36,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 64,
        "source": 37,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 42,
        "provenance": "authored",
        "reference": 116,
        "source": 38,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 65,
        "source": 39,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 44,
        "provenance": "authored",
        "reference": 118,
        "source": 40,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 66,
        "source": 41,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 46,
        "provenance": "authored",
        "reference": 119,
        "source": 1,
        "target": {
          "node": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 47,
        "provenance": "authored",
        "reference": 120,
        "source": 1,
        "target": {
          "node": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 49,
        "provenance": "authored",
        "reference": 121,
        "source": 2,
        "target": {
          "node": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 50,
        "provenance": "authored",
        "reference": 122,
        "source": 2,
        "target": {
          "node": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 52,
        "provenance": "authored",
        "reference": 123,
        "source": 3,
        "target": {
          "node": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 53,
        "provenance": "authored",
        "reference": 124,
        "source": 3,
        "target": {
          "node": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 55,
        "provenance": "authored",
        "reference": 125,
        "source": 4,
        "target": {
          "node": 29,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 56,
        "provenance": "authored",
        "reference": 126,
        "source": 4,
        "target": {
          "node": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 58,
        "provenance": "authored",
        "reference": 127,
        "source": 5,
        "target": {
          "node": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 59,
        "provenance": "authored",
        "reference": 128,
        "source": 5,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 61,
        "provenance": "authored",
        "reference": 129,
        "source": 6,
        "target": {
          "node": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 62,
        "provenance": "authored",
        "reference": 130,
        "source": 6,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 64,
        "provenance": "authored",
        "reference": 131,
        "source": 7,
        "target": {
          "node": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 65,
        "provenance": "authored",
        "reference": 132,
        "source": 7,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 67,
        "provenance": "authored",
        "reference": 133,
        "source": 8,
        "target": {
          "node": 37,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 68,
        "provenance": "authored",
        "reference": 134,
        "source": 8,
        "target": {
          "node": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 70,
        "provenance": "authored",
        "reference": 135,
        "source": 9,
        "target": {
          "node": 39,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 71,
        "provenance": "authored",
        "reference": 136,
        "source": 9,
        "target": {
          "node": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 73,
        "provenance": "authored",
        "reference": 137,
        "source": 10,
        "target": {
          "node": 41,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 74,
        "provenance": "authored",
        "reference": 138,
        "source": 10,
        "target": {
          "node": 21,
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
