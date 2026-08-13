# META
~~~ini
description=KerML Packet: Packets
type=file
~~~
# SOURCE
~~~kerml
private import ScalarValues::*;
private import Time::DateTime;
package Packets {
	
	feature 'packet header' { }
	
	feature 'packet data field' {	
		feature 'packet secondary header' redefines 'packet header';
		feature 'user data field';
	}
	
	class 'Data Packet' { 
		feature 'packet primary header' redefines 'packet header' {
			feature 'packet version number': Integer;
			feature 'packet identification': String;
			feature 'packet data length': Integer;
		}
		feature redefines 'packet data field';
	}
	
	class 'Thermal Data Packet' specializes 'Data Packet' {
		feature 'packet data field' redefines Packets::'packet data field'{
			feature 'packet secondary header' redefines 'packet header' {
				feature 'packet timestamp': DateTime;
				feature 'telemetry packet type': String;
			}
			
			feature 'user data field' redefines Packets::'packet data field'::'user data field' {
				feature timestamp: DateTime;
				feature temperature: Real;
			}
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/packets.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 15) (end 0 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 1) (end 4 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 46) (end 7 61))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 12 2) (end 17 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 17 2) (end 18 1))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 21 2) (end 32 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:2cc65cd9d54e1a2b7ea0f95816ad1053c065af4d476a2d3aefacda1e9a8d356a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/packets.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/packets.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::DateTime") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Data Packet"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "packet header"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::user data field"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/packets.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (kind specialization) (ordinal 0))
      (authored-target "Data Packet")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet")))))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0))
      (authored-target "packet header")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/packets.md") (range (start 0 15) (end 0 30)) (probe (position 0 15))
    (reference (id (source (node (document "memory://snapshot/packets.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packets.md") (range (start 1 15) (end 1 29)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/packets.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Time::DateTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packets.md") (range (start 20 41) (end 20 54)) (probe (position 20 41))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (kind specialization) (ordinal 0) (authored-target "Data Packet")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet")))))
  )
  (query (document "memory://snapshot/packets.md") (range (start 7 46) (end 7 61)) (probe (position 7 46))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0) (authored-target "packet header")
      (outcome (status unresolved)))
  )
)
~~~
