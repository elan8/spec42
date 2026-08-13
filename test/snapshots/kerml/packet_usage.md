# META
~~~ini
description=KerML Packet: PacketUsage
type=file
~~~
# SOURCE
~~~kerml
private import Packets::*;
private import ScalarValues::Real;
package 'Packet Usage' {
	
	feature packet1: 'Thermal Data Packet';
	feature packet2: 'Thermal Data Packet';
	feature packet3: 'Thermal Data Packet' {
		feature 'special data field' redefines 'packet data field'{
			feature :>> 'user data field' {
				feature 'special data': Real;
			}
		}
	}
	
}
	
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/packet_usage.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 15) (end 0 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 1) (end 4 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 1) (end 5 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 18) (end 6 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 41) (end 7 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 15) (end 8 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 28) (end 9 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:3355488f143ea1f1ed163b4d5fe64e9bcdc5076435e5801fad2f2806b2c637ac") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Packets") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thermal Data Packet"))))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "packet data field"))))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "user data field"))))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field::::special data"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Packets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thermal Data Packet")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field"))) (kind redefinition) (ordinal 0))
      (authored-target "packet data field")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "user data field")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field::::special data"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/packet_usage.md") (range (start 0 15) (end 0 25)) (probe (position 0 15))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Packets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packet_usage.md") (range (start 1 15) (end 1 33)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packet_usage.md") (range (start 6 18) (end 6 39)) (probe (position 6 18))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3"))) (kind featureTyping) (ordinal 0) (authored-target "Thermal Data Packet")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packet_usage.md") (range (start 7 41) (end 7 60)) (probe (position 7 41))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field"))) (kind redefinition) (ordinal 0) (authored-target "packet data field")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packet_usage.md") (range (start 8 15) (end 8 32)) (probe (position 8 15))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "user data field")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packet_usage.md") (range (start 9 28) (end 9 32)) (probe (position 9 28))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field::::special data"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
)
~~~
