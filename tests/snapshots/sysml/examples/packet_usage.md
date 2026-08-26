# META
~~~ini
description=SysML Example (Packet): PacketUsage
type=file
~~~
# SOURCE
~~~sysml
package 'Packet Usage' {
	public import Packets::*;
	private import ScalarValues::Real;
	
	part packet1: 'Thermal Data Packet';
	part packet2: 'Thermal Data Packet';
	part packet3: 'Thermal Data Packet' {
		attribute 'special data field' redefines 'packet data field'{
			attribute redefines 'user data field' {
				attribute 'special data': Real;
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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 15) (end 1 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 15) (end 4 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 15) (end 5 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 15) (end 6 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 43) (end 7 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 23) (end 8 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 30) (end 9 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:77b2e143982006a0827a184bf441f5e1ac0b291f4e74bf124afed2eeac65e8ae") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Packets") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thermal Data Packet")))))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thermal Data Packet")))))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thermal Data Packet")))))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "packet data field")))))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (named (kind part) (name "packet3")) (named (kind attribute) (name "special data field")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "user data field")))))
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (named (kind part) (name "packet3")) (named (kind attribute) (name "special data field")) (anonymous (kind attribute) (ordinal 0)) (named (kind attribute) (name "special data"))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Packets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thermal Data Packet")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thermal Data Packet")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thermal Data Packet")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field"))) (kind redefinition) (ordinal 0))
      (authored-target "packet data field")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (named (kind part) (name "packet3")) (named (kind attribute) (name "special data field")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "user data field")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (named (kind part) (name "packet3")) (named (kind attribute) (name "special data field")) (anonymous (kind attribute) (ordinal 0)) (named (kind attribute) (name "special data"))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field"))) (target (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (named (kind part) (name "packet3")) (named (kind attribute) (name "special data field")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (named (kind part) (name "packet3")) (named (kind attribute) (name "special data field")) (anonymous (kind attribute) (ordinal 0)) (named (kind attribute) (name "special data"))))) (target (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (named (kind part) (name "packet3")) (named (kind attribute) (name "special data field")) (anonymous (kind attribute) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field")))
      (featured-by (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3")))
    )
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (named (kind part) (name "packet3")) (named (kind attribute) (name "special data field")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field")))
    )
    (declaration (id (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (named (kind part) (name "packet3")) (named (kind attribute) (name "special data field")) (anonymous (kind attribute) (ordinal 0)) (named (kind attribute) (name "special data")))))
      (featured-by (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (named (kind part) (name "packet3")) (named (kind attribute) (name "special data field")) (anonymous (kind attribute) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/packet_usage.md") (range (start 1 15) (end 1 25)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Packets")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packet_usage.md") (range (start 2 16) (end 2 34)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packet_usage.md") (range (start 4 15) (end 4 36)) (probe (position 4 15))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet1"))) (kind featureTyping) (ordinal 0) (authored-target "Thermal Data Packet")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packet_usage.md") (range (start 5 15) (end 5 36)) (probe (position 5 15))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet2"))) (kind featureTyping) (ordinal 0) (authored-target "Thermal Data Packet")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packet_usage.md") (range (start 6 15) (end 6 36)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3"))) (kind featureTyping) (ordinal 0) (authored-target "Thermal Data Packet")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packet_usage.md") (range (start 7 43) (end 7 62)) (probe (position 7 43))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (qualified-name "Packet Usage::packet3::special data field"))) (kind redefinition) (ordinal 0) (authored-target "packet data field")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packet_usage.md") (range (start 8 23) (end 8 40)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (named (kind part) (name "packet3")) (named (kind attribute) (name "special data field")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "user data field")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packet_usage.md") (range (start 9 30) (end 9 34)) (probe (position 9 30))
    (reference (id (source (node (document "memory://snapshot/packet_usage.md") (path (named (kind package) (name "Packet Usage")) (named (kind part) (name "packet3")) (named (kind attribute) (name "special data field")) (anonymous (kind attribute) (ordinal 0)) (named (kind attribute) (name "special data"))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
)
~~~
