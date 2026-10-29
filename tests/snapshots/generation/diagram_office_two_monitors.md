# META
~~~ini
description=An unconnected second monitor must not remove the first monitor's interconnections
type=generate
libraries=standard
plugin=native:diagram
viewKind=interconnection-view
viewDocument=model.sysml
viewQualifiedName=Office::connections
~~~
# SOURCE
## model.sysml
~~~sysml
package Office {
    private import StandardViewDefinitions::*;

    port def VideoPort;
    port def PowerPort;

    part def Laptop {
        port video : VideoPort;
        port power : PowerPort;
    }
    part def Monitor {
        port video : ~VideoPort;
        port power : ~PowerPort;
    }
    part def Socket {
        port power : PowerPort;
    }
    part def Workplace {
        part laptop : Laptop;
        part monitor1 : Monitor;
        part monitor2 : Monitor;
        part socket : Socket;
        connect laptop.video to monitor1.video;
        connect monitor1.power to socket.power;
        connect laptop.power to socket.power;
    }
    view connections : InterconnectionView {
        expose Workplace;
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:554ef2d294e625198f7ba058df8fbfc2153fa5564c5e6fc7d13a05027641aeea") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VideoPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VideoPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~PowerPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "VideoPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~VideoPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "laptop::video")) (memberAccessOperand (reference "monitor1::video")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "monitor1::power")) (memberAccessOperand (reference "socket::power")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "laptop::power")) (memberAccessOperand (reference "socket::power")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Laptop")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Monitor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Monitor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Socket")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::connections"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Workplace")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video"))) (kind featureTyping) (ordinal 0))
      (authored-target "VideoPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (kind featureTyping) (ordinal 0))
      (authored-target "VideoPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "laptop::video")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "monitor1::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "laptop::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "monitor1::video")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "socket::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "socket::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop"))) (kind featureTyping) (ordinal 0))
      (authored-target "Laptop")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Monitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Monitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket"))) (kind featureTyping) (ordinal 0))
      (authored-target "Socket")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::connections"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Workplace")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::connections"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::connections"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "VideoPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "VideoPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::connections"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::connections"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "VideoPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "VideoPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "VideoPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind port-def) (name "VideoPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::connections")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::connections")))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop")) (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video"))) "laptop::video")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1")) (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) "monitor1::video")))
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1")) (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) "monitor1::power")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket")) (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) "socket::power")))
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop")) (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power"))) "laptop::power")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket")) (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) "socket::power")))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 21) (end 8 30)) (probe (position 8 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power"))) (kind featureTyping) (ordinal 0) (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 21) (end 7 30)) (probe (position 7 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video"))) (kind featureTyping) (ordinal 0) (authored-target "VideoPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 22) (end 12 31)) (probe (position 12 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) (kind featureTyping) (ordinal 0) (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 22) (end 11 31)) (probe (position 11 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (kind featureTyping) (ordinal 0) (authored-target "VideoPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 21) (end 15 30)) (probe (position 15 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (kind featureTyping) (ordinal 0) (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 22 16) (end 22 28)) (probe (position 22 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "laptop::video")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::video")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 23 16) (end 23 30)) (probe (position 23 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "monitor1::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 24 16) (end 24 28)) (probe (position 24 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0) (authored-target "laptop::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 22 32) (end 22 46)) (probe (position 22 32))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "monitor1::video")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 23 34) (end 23 46)) (probe (position 23 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1) (authored-target "socket::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 24 32) (end 24 44)) (probe (position 24 32))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 1) (authored-target "socket::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 22) (end 18 28)) (probe (position 18 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop"))) (kind featureTyping) (ordinal 0) (authored-target "Laptop")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 19 24) (end 19 31)) (probe (position 19 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1"))) (kind featureTyping) (ordinal 0) (authored-target "Monitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 20 24) (end 20 31)) (probe (position 20 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2"))) (kind featureTyping) (ordinal 0) (authored-target "Monitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 21 22) (end 21 28)) (probe (position 21 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket"))) (kind featureTyping) (ordinal 0) (authored-target "Socket")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 26 23) (end 26 42)) (probe (position 26 23))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::connections"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 27 15) (end 27 24)) (probe (position 27 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Workplace")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:c3699d363e8df1db7a66023c9d16211ad9c29c1bdae3e5f1767a67dc2ec03e9a",
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
        7,
        13,
        7,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        21,
        7,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        13,
        8,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        21,
        8,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        13,
        11,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        22,
        11,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        13,
        12,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        22,
        12,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        13,
        15,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        21,
        15,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        13,
        17,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        13,
        18,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        22,
        18,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        13,
        19,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        24,
        19,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        20,
        13,
        20,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        20,
        24,
        20,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        13,
        21,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        22,
        21,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        8,
        22,
        47
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        16,
        22,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        32,
        22,
        46
      ]
    },
    {
      "document": 0,
      "range": [
        23,
        8,
        23,
        47
      ]
    },
    {
      "document": 0,
      "range": [
        23,
        16,
        23,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        23,
        34,
        23,
        46
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        8,
        24,
        45
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        16,
        24,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        32,
        24,
        44
      ]
    },
    {
      "document": 0,
      "range": [
        26,
        9,
        26,
        20
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Laptop"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Laptop::power"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Laptop::video"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Monitor"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Monitor::power"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Monitor::video"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::PowerPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Socket"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Socket::power"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::VideoPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Workplace"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Workplace::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Workplace::laptop"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Workplace::monitor1"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Workplace::monitor2"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::Workplace::socket"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Office::connections"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Office::Workplace",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Office::Workplace",
      "source": 22,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Office::Workplace",
      "source": 25,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "connection",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "connection",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "connection",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 57,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 62,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 3,
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
      "ordinal": 7,
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
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "memberAccessOperand",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "memberAccessOperand",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "memberAccessOperand",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "memberAccessOperand",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "memberAccessOperand",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "memberAccessOperand",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "noncomposite-membership",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "noncomposite-membership",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "typing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "noncomposite-membership",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "noncomposite-membership",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 53,
      "relationshipKind": "typing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "typing",
      "source": 15
    }
  ],
  "selectedView": {
    "reference": 16,
    "kind": "interconnection-view",
    "name": "connections",
    "source": 28
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 4,
        "provenance": "authored",
        "reference": 30,
        "source": 0,
        "target": 4
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 2,
        "origin": 5,
        "provenance": "implied",
        "reference": 43,
        "source": 4,
        "target": 5
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 6,
        "provenance": "implied",
        "reference": 44,
        "source": 4,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 7,
        "provenance": "authored",
        "reference": 31,
        "source": 0,
        "target": 7
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 8,
        "origin": 8,
        "provenance": "implied",
        "reference": 52,
        "source": 7,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 1,
        "provenance": "authored",
        "reference": 32,
        "source": 0,
        "target": 1
      },
      {
        "kind": "connector",
        "navigation": 20,
        "origin": 1,
        "provenance": "authored",
        "reference": 22,
        "source": 6,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 22,
        "origin": 2,
        "provenance": "authored",
        "reference": 33,
        "source": 0,
        "target": 2
      },
      {
        "kind": "connector",
        "navigation": 23,
        "origin": 2,
        "provenance": "authored",
        "reference": 24,
        "source": 10,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 25,
        "origin": 3,
        "provenance": "authored",
        "reference": 34,
        "source": 0,
        "target": 3
      },
      {
        "kind": "connector",
        "navigation": 26,
        "origin": 3,
        "provenance": "authored",
        "reference": 20,
        "source": 5,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 9,
        "provenance": "authored",
        "reference": 35,
        "source": 0,
        "target": 9
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 6,
        "origin": 10,
        "provenance": "implied",
        "reference": 46,
        "source": 9,
        "target": 10
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 4,
        "origin": 11,
        "provenance": "implied",
        "reference": 47,
        "source": 9,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 12,
        "provenance": "authored",
        "reference": 36,
        "source": 0,
        "target": 12
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 6,
        "origin": 13,
        "provenance": "implied",
        "reference": 49,
        "source": 12,
        "target": 13
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 4,
        "origin": 14,
        "provenance": "implied",
        "reference": 50,
        "source": 12,
        "target": 14
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
        3
      ],
      "parts": [
        0,
        4,
        7,
        9,
        12
      ],
      "portFamilies": [
        {
          "connectors": [
            2,
            3
          ],
          "general": [
            {
              "label": "PowerPort",
              "reference": 6
            }
          ],
          "types": [
            {
              "label": "PowerPort",
              "reference": 6
            }
          ]
        },
        {
          "connectors": [
            1
          ],
          "general": [
            {
              "label": "VideoPort",
              "reference": 9
            }
          ],
          "types": [
            {
              "label": "VideoPort",
              "reference": 9
            }
          ]
        }
      ],
      "ports": [
        5,
        6,
        8,
        10,
        11,
        13,
        14
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              4,
              7,
              9,
              12
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              1,
              2,
              3
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartDefinition",
        "multiplicity": null,
        "name": "Workplace",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:d88654e8e5e745ff0b055bac68d799c99850fdac6e24639cae38df6c3c768a62",
        "reference": 10,
        "source": 10,
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
        "presentationKey": "p:457431d1e6efae1da271f79d40bcbac6ef5b2fb02a08bc880cde03f22e9dcc92",
        "reference": 17,
        "source": 19,
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
        "presentationKey": "p:87d6da4602701c6f7189318af24d9cf3dc636540b94b7c81baed96bdc490074b",
        "reference": 18,
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
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:2a470b68d35b6d0b93e7975715acc3b450150bbb61d5eb0c4d97a06d471c4dd9",
        "reference": 19,
        "source": 25,
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
              5,
              6
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "laptop",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:43b6eb733d88ea8c23675a52019eb44d568beec5a6f0277f0c920da8b612ed85",
        "reference": 12,
        "source": 11,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Laptop",
              "reference": 0
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
        "owner": 4,
        "presentationKey": "p:975febd1fd6a9d6f80a8d8278fe391a86ec6e7e6722b29cc7cd7780822faa0ec",
        "reference": 1,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 6
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
        "name": "video",
        "notationRole": "usage",
        "owner": 4,
        "presentationKey": "p:849cbf7610282ebe387fc64c43d019d3df928e2aa458b64df07af2405501f9db",
        "reference": 2,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "VideoPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              8
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "socket",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:827a9bfa087e5358a7048abb635ade90b62301c11611e318abb9ffdf9406dade",
        "reference": 15,
        "source": 17,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Socket",
              "reference": 7
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
        "owner": 7,
        "presentationKey": "p:f7c87b003e8f762d1f72130c3c7a711c148bf0fd969765c5a019d13ff96e1abd",
        "reference": 8,
        "source": 8,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 6
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              10,
              11
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "monitor1",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:d555e006759875b4ab57c5b712e6e7792829f049164619e95e691ce6339cd291",
        "reference": 13,
        "source": 13,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Monitor",
              "reference": 3
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
        "owner": 9,
        "presentationKey": "p:812978e1977aefb10d6a4c4ee5c7d71610fca86b5195f8ddb6f24db3b15a53e2",
        "reference": 4,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 6
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
        "name": "video",
        "notationRole": "usage",
        "owner": 9,
        "presentationKey": "p:067dbd882472ca34dcb73048fb3219185cf0298afe0551d1ec885f9f6aff37d8",
        "reference": 5,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "VideoPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              13,
              14
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "monitor2",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:9e58d53e7008aa009cd891cf5cc70eb631ceed8f2cc1581cdcc9981af32f8f97",
        "reference": 14,
        "source": 15,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Monitor",
              "reference": 3
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
        "owner": 12,
        "presentationKey": "p:b03d98a2ac66db3bc9fa8d9c18a97b5304ff0ee36cda15b741b4456d5a9d5c84",
        "reference": 4,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 6
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
        "name": "video",
        "notationRole": "usage",
        "owner": 12,
        "presentationKey": "p:df0e84d3b32bb5519b51ee951b5b775e71a4308f8b6d529c3f39892131520389",
        "reference": 5,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "VideoPort",
              "reference": 9
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 12,
        "provenance": "authored",
        "reference": 45,
        "source": 4,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 21,
        "source": 5,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 23,
        "source": 6,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 18,
        "provenance": "authored",
        "reference": 53,
        "source": 7,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 29,
        "source": 8,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 20,
        "provenance": "authored",
        "reference": 37,
        "source": 1,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 21,
        "provenance": "authored",
        "reference": 38,
        "source": 1,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 23,
        "provenance": "authored",
        "reference": 39,
        "source": 2,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 24,
        "provenance": "authored",
        "reference": 40,
        "source": 2,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 26,
        "provenance": "authored",
        "reference": 41,
        "source": 3,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 27,
        "provenance": "authored",
        "reference": 42,
        "source": 3,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 14,
        "provenance": "authored",
        "reference": 48,
        "source": 9,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 7,
        "provenance": "authored",
        "reference": 25,
        "source": 10,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 27,
        "source": 11,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 16,
        "provenance": "authored",
        "reference": 51,
        "source": 12,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 7,
        "provenance": "authored",
        "reference": 26,
        "source": 13,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 28,
        "source": 14,
        "target": {
          "reference": 9,
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
