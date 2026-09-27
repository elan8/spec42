# META
~~~ini
description=An unconnected second monitor must not remove the first monitor's interconnections
type=generate
libraries=standard
plugin=repository:diagram
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:38af0fbc4bf572d5c6a1c082136465cf961e2352b5eda799f6ccf57a97233da1") (admitted (standard-library 94)))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort"))) (kind port-def) (membership (kind owning) (visibility default)))
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
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::VideoPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::video")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::PowerPort")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop::power")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor::power")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket::power")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (provenance implied))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (provenance implied))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Office")) (named (kind part-def) (name "Workplace")) (anonymous (kind bare-connect) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (provenance implied))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::laptop")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Laptop")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor1")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::monitor2")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Monitor")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace::socket")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Workplace")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Office::Socket")) (scopes any))
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
  "schemaVersion": 5,
  "modelDigest": "blake3:08e5d58e6474d0efb7c53a69ca4eba7c9f89a44a3838d8af52509466743701a0",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/connections.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/parts.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/ports.md",
      "sourceDomain": "standard-library"
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
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Connections::connections"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part::ownedPorts"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Parts::parts"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Ports::ports"
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
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 4,
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
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 8,
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
      "ordinal": 35,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "typeFeaturing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 52,
      "relationshipKind": "typeFeaturing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "typeFeaturing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 15,
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
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "memberAccessOperand",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "memberAccessOperand",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "memberAccessOperand",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 25,
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
      "ordinal": 29,
      "relationshipKind": "memberAccessOperand",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "typeFeaturing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "typeFeaturing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "typeFeaturing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "typing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "typing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "typing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "typeFeaturing",
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
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "typeFeaturing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "typing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "subsetting",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 44,
      "relationshipKind": "typeFeaturing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "typing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "subsetting",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "typeFeaturing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 12,
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
        "reference": 56,
        "source": 0,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 5,
        "provenance": "implied",
        "reference": 76,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 0,
        "origin": 6,
        "provenance": "implied",
        "reference": 77,
        "source": 4,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 7,
        "provenance": "authored",
        "reference": 57,
        "source": 0,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 8,
        "provenance": "implied",
        "reference": 91,
        "source": 7,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 1,
        "provenance": "authored",
        "reference": 58,
        "source": 0,
        "target": 1
      },
      {
        "kind": "connector",
        "navigation": 20,
        "origin": 1,
        "provenance": "authored",
        "reference": 30,
        "source": 6,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 22,
        "origin": 2,
        "provenance": "authored",
        "reference": 59,
        "source": 0,
        "target": 2
      },
      {
        "kind": "connector",
        "navigation": 23,
        "origin": 2,
        "provenance": "authored",
        "reference": 35,
        "source": 10,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 25,
        "origin": 3,
        "provenance": "authored",
        "reference": 60,
        "source": 0,
        "target": 3
      },
      {
        "kind": "connector",
        "navigation": 26,
        "origin": 3,
        "provenance": "authored",
        "reference": 25,
        "source": 5,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 9,
        "provenance": "authored",
        "reference": 61,
        "source": 0,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 10,
        "provenance": "implied",
        "reference": 81,
        "source": 9,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 11,
        "provenance": "implied",
        "reference": 82,
        "source": 9,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 12,
        "provenance": "authored",
        "reference": 62,
        "source": 0,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 13,
        "provenance": "implied",
        "reference": 86,
        "source": 12,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 14,
        "provenance": "implied",
        "reference": 87,
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
        "metaclass": "PartDefinition",
        "name": "Workplace",
        "notationRole": "definition",
        "owner": null,
        "reference": 10,
        "source": 10,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 22,
        "source": 19,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 23,
        "source": 22,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 24,
        "source": 25,
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
        "metaclass": "PartUsage",
        "name": "laptop",
        "notationRole": "usage",
        "owner": 0,
        "reference": 12,
        "source": 11,
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
        "metaclass": "PortUsage",
        "name": "power",
        "notationRole": "usage",
        "owner": 4,
        "reference": 1,
        "source": 2,
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
        "metaclass": "PortUsage",
        "name": "video",
        "notationRole": "usage",
        "owner": 4,
        "reference": 2,
        "source": 0,
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
        "metaclass": "PartUsage",
        "name": "socket",
        "notationRole": "usage",
        "owner": 0,
        "reference": 15,
        "source": 17,
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
        "metaclass": "PortUsage",
        "name": "power",
        "notationRole": "usage",
        "owner": 7,
        "reference": 8,
        "source": 8,
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
        "metaclass": "PartUsage",
        "name": "monitor1",
        "notationRole": "usage",
        "owner": 0,
        "reference": 13,
        "source": 13,
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
        "metaclass": "PortUsage",
        "name": "power",
        "notationRole": "usage",
        "owner": 9,
        "reference": 4,
        "source": 6,
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
        "metaclass": "PortUsage",
        "name": "video",
        "notationRole": "usage",
        "owner": 9,
        "reference": 5,
        "source": 4,
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
        "metaclass": "PartUsage",
        "name": "monitor2",
        "notationRole": "usage",
        "owner": 0,
        "reference": 14,
        "source": 15,
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
        "metaclass": "PortUsage",
        "name": "power",
        "notationRole": "usage",
        "owner": 12,
        "reference": 4,
        "source": 6,
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
        "metaclass": "PortUsage",
        "name": "video",
        "notationRole": "usage",
        "owner": 12,
        "reference": 5,
        "source": 4,
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
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 63,
        "source": 0,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 12,
        "provenance": "authored",
        "reference": 80,
        "source": 4,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 78,
        "source": 4,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 79,
        "source": 4,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 29,
        "source": 5,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 26,
        "source": 5,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 27,
        "source": 5,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 28,
        "source": 5,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 34,
        "source": 6,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 31,
        "source": 6,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 32,
        "source": 6,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 33,
        "source": 6,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 18,
        "provenance": "authored",
        "reference": 94,
        "source": 7,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 92,
        "source": 7,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 93,
        "source": 7,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 55,
        "source": 8,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 52,
        "source": 8,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 53,
        "source": 8,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 54,
        "source": 8,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": null,
        "provenance": "implied",
        "reference": 73,
        "source": 1,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 20,
        "provenance": "authored",
        "reference": 64,
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
        "reference": 65,
        "source": 1,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 70,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": null,
        "provenance": "implied",
        "reference": 74,
        "source": 2,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 23,
        "provenance": "authored",
        "reference": 66,
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
        "reference": 67,
        "source": 2,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 71,
        "source": 2,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": null,
        "provenance": "implied",
        "reference": 75,
        "source": 3,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 26,
        "provenance": "authored",
        "reference": 68,
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
        "reference": 69,
        "source": 3,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 72,
        "source": 3,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 14,
        "provenance": "authored",
        "reference": 85,
        "source": 9,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 83,
        "source": 9,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 84,
        "source": 9,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 7,
        "provenance": "authored",
        "reference": 42,
        "source": 10,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 36,
        "source": 10,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 37,
        "source": 10,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 40,
        "source": 10,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 50,
        "source": 11,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 44,
        "source": 11,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 45,
        "source": 11,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 48,
        "source": 11,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 16,
        "provenance": "authored",
        "reference": 90,
        "source": 12,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 88,
        "source": 12,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 89,
        "source": 12,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 7,
        "provenance": "authored",
        "reference": 43,
        "source": 13,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 38,
        "source": 13,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 39,
        "source": 13,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 41,
        "source": 13,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 51,
        "source": 14,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 46,
        "source": 14,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 47,
        "source": 14,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 49,
        "source": 14,
        "target": {
          "reference": 3,
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
