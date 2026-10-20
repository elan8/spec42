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
  "schemaVersion": 6,
  "modelDigest": "blake3:627445e34db6abd10c657849dda6f474bec7b3d15986c3865d0bae705b3bc71c",
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
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Laptop"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Laptop::power"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Laptop::video"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Monitor"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Monitor::power"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Monitor::video"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::PowerPort"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Socket"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Socket::power"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::VideoPort"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Workplace"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Workplace::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Workplace::laptop"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Workplace::monitor1"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Workplace::monitor2"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::Workplace::socket"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Office::connections"
    },
    {
      "kind": "source-anchor",
      "source": 19,
      "ownerQualifiedName": "Office::Workplace",
      "metaclass": "ConnectionUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 22,
      "ownerQualifiedName": "Office::Workplace",
      "metaclass": "ConnectionUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 25,
      "ownerQualifiedName": "Office::Workplace",
      "metaclass": "ConnectionUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "connection",
      "ordinal": 10
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "typing",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "connection",
      "ordinal": 6
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "typing",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "connection",
      "ordinal": 8
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "typing",
      "ordinal": 43
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "typing",
      "ordinal": 57
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "typing",
      "ordinal": 48
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "typing",
      "ordinal": 62
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "typing",
      "ordinal": 17
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "containment",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "containment",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "containment",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "containment",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "containment",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "containment",
      "ordinal": 11
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "containment",
      "ordinal": 14
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 21
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 22
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 27
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 28
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 33
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 34
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "typing",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 12
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "typing",
      "ordinal": 39
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 15
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 16
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "typing",
      "ordinal": 53
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "typing",
      "ordinal": 13
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
    "kind": "interconnection-view",
    "exposedRoots": [
      0
    ],
    "nodes": [
      {
        "reference": 10,
        "metaclass": "PartDefinition",
        "notationRole": "definition",
        "name": "Workplace",
        "typing": {
          "status": "absent"
        },
        "owner": null,
        "source": 10,
        "compartments": [
          {
            "kind": "parts",
            "provenance": "direct",
            "members": [
              4,
              7,
              9,
              12
            ]
          },
          {
            "kind": "connections",
            "provenance": "direct",
            "members": [
              1,
              2,
              3
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 17,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 19,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 18,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 22,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 19,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 25,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 12,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "laptop",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 0,
              "label": "Laptop"
            }
          ]
        },
        "owner": 0,
        "source": 11,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              5,
              6
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 1,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "power",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 4,
        "source": 2,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 2,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "video",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "VideoPort"
            }
          ]
        },
        "owner": 4,
        "source": 0,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 15,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "socket",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 7,
              "label": "Socket"
            }
          ]
        },
        "owner": 0,
        "source": 17,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              8
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 8,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "power",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 7,
        "source": 8,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 13,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "monitor1",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 3,
              "label": "Monitor"
            }
          ]
        },
        "owner": 0,
        "source": 13,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              10,
              11
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 4,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "power",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 9,
        "source": 6,
        "compartments": [],
        "direction": null,
        "conjugated": true,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 5,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "video",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "VideoPort"
            }
          ]
        },
        "owner": 9,
        "source": 4,
        "compartments": [],
        "direction": null,
        "conjugated": true,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 14,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "monitor2",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 3,
              "label": "Monitor"
            }
          ]
        },
        "owner": 0,
        "source": 15,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              13,
              14
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 4,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "power",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 12,
        "source": 6,
        "compartments": [],
        "direction": null,
        "conjugated": true,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 5,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "video",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "VideoPort"
            }
          ]
        },
        "owner": 12,
        "source": 4,
        "compartments": [],
        "direction": null,
        "conjugated": true,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      }
    ],
    "relationships": [
      {
        "reference": 45,
        "source": 4,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 12
      },
      {
        "reference": 21,
        "source": 5,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 23,
        "source": 6,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 53,
        "source": 7,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 7
        },
        "provenance": "authored",
        "navigation": 18
      },
      {
        "reference": 29,
        "source": 8,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 37,
        "source": 1,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 6
        },
        "provenance": "authored",
        "navigation": 20
      },
      {
        "reference": 38,
        "source": 1,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 11
        },
        "provenance": "authored",
        "navigation": 21
      },
      {
        "reference": 39,
        "source": 2,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 10
        },
        "provenance": "authored",
        "navigation": 23
      },
      {
        "reference": 40,
        "source": 2,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 8
        },
        "provenance": "authored",
        "navigation": 24
      },
      {
        "reference": 41,
        "source": 3,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 5
        },
        "provenance": "authored",
        "navigation": 26
      },
      {
        "reference": 42,
        "source": 3,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 8
        },
        "provenance": "authored",
        "navigation": 27
      },
      {
        "reference": 48,
        "source": 9,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 3
        },
        "provenance": "authored",
        "navigation": 14
      },
      {
        "reference": 25,
        "source": 10,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 27,
        "source": 11,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 51,
        "source": 12,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 3
        },
        "provenance": "authored",
        "navigation": 16
      },
      {
        "reference": 26,
        "source": 13,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 28,
        "source": 14,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 5
      }
    ],
    "edges": [
      {
        "reference": 30,
        "source": 0,
        "target": 4,
        "origin": 4,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 43,
        "source": 4,
        "target": 5,
        "origin": 5,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 2
      },
      {
        "reference": 44,
        "source": 4,
        "target": 6,
        "origin": 6,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 0
      },
      {
        "reference": 31,
        "source": 0,
        "target": 7,
        "origin": 7,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 17
      },
      {
        "reference": 52,
        "source": 7,
        "target": 8,
        "origin": 8,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 8
      },
      {
        "reference": 32,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 22,
        "source": 6,
        "target": 11,
        "origin": 1,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 20
      },
      {
        "reference": 33,
        "source": 0,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 22
      },
      {
        "reference": 24,
        "source": 10,
        "target": 8,
        "origin": 2,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 23
      },
      {
        "reference": 34,
        "source": 0,
        "target": 3,
        "origin": 3,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 25
      },
      {
        "reference": 20,
        "source": 5,
        "target": 8,
        "origin": 3,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 26
      },
      {
        "reference": 35,
        "source": 0,
        "target": 9,
        "origin": 9,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 13
      },
      {
        "reference": 46,
        "source": 9,
        "target": 10,
        "origin": 10,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 6
      },
      {
        "reference": 47,
        "source": 9,
        "target": 11,
        "origin": 11,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 4
      },
      {
        "reference": 36,
        "source": 0,
        "target": 12,
        "origin": 12,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 15
      },
      {
        "reference": 49,
        "source": 12,
        "target": 13,
        "origin": 13,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 6
      },
      {
        "reference": 50,
        "source": 12,
        "target": 14,
        "origin": 14,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 4
      }
    ],
    "metadata": {
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
      ],
      "connectors": [
        1,
        2,
        3
      ]
    },
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
