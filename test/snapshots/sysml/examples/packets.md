# META
~~~ini
description=SysML Example (Packet): Packets
type=file
~~~
# SOURCE
~~~sysml
package Packets {
	private import ScalarValues::*;
	private import Time::DateTime;
	
	attribute 'packet header' { }
	
	attribute 'packet data field' {	
		attribute 'packet secondary header' redefines 'packet header';
		attribute 'user data field';
	}
	
	part def 'Data Packet' {
		attribute 'packet primary header' redefines 'packet header' {
			attribute 'packet version number': Integer;
			attribute 'packet identification': String;
			attribute 'packet data length': Integer;
		}
		attribute redefines 'packet data field';
	}
	
	part def 'Thermal Data Packet' :> 'Data Packet' {
		attribute 'packet data field' redefines Packets::'packet data field'{
			attribute 'packet secondary header' redefines 'packet header' {
				attribute 'packet timestamp': DateTime;
				attribute 'telemetry packet type': String;
			}
			
			attribute 'user data field' redefines Packets::'packet data field'::'user data field' {
				attribute timestamp: DateTime;
				attribute temperature: Real;
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
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 38) (end 13 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 38) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 35) (end 15 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 34) (end 23 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 39) (end 24 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 25) (end 28 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 27) (end 29 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3ba94dd195fc2b4f3c113127b3affb29df4c0c3753d6b604c88d7541c2ee2a18") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::DateTime") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind part-def) (name "Data Packet")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "packet data field"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "packet header"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet data length"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet identification"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet version number"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Data Packet"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Packets::packet data field"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "packet header"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::packet timestamp"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DateTime"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::telemetry packet type"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Packets::packet data field::user data field"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::temperature"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::timestamp"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DateTime"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "packet header"))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::user data field"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (anonymous (kind import) (ordinal 1)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind part-def) (name "Data Packet")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "packet data field")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field")))))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (kind redefinition) (ordinal 0))
      (authored-target "packet header")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")))))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet data length"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet identification"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet version number"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (kind specialization) (ordinal 0))
      (authored-target "Data Packet")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet")))))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (kind redefinition) (ordinal 0))
      (authored-target "Packets::packet data field")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field")))))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0))
      (authored-target "packet header")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")))))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::packet timestamp"))) (kind featureTyping) (ordinal 0))
      (authored-target "DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::telemetry packet type"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field"))) (kind redefinition) (ordinal 0))
      (authored-target "Packets::packet data field::user data field")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::user data field")))))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::temperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::timestamp"))) (kind featureTyping) (ordinal 0))
      (authored-target "DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0))
      (authored-target "packet header")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind part-def) (name "Data Packet")) (anonymous (kind attribute) (ordinal 0)))))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind part-def) (name "Data Packet")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::user data field"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/packets.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packets.md") (range (start 2 16) (end 2 30)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (anonymous (kind import) (ordinal 1)))))) (kind membershipImport) (ordinal 0) (authored-target "Time::DateTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packets.md") (range (start 17 22) (end 17 41)) (probe (position 17 22))
    (reference (id (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind part-def) (name "Data Packet")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "packet data field")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field")))))
  )
  (query (document "memory://snapshot/packets.md") (range (start 12 46) (end 12 61)) (probe (position 12 46))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (kind redefinition) (ordinal 0) (authored-target "packet header")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")))))
  )
  (query (document "memory://snapshot/packets.md") (range (start 15 35) (end 15 42)) (probe (position 15 35))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet data length"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packets.md") (range (start 14 38) (end 14 44)) (probe (position 14 38))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet identification"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packets.md") (range (start 13 38) (end 13 45)) (probe (position 13 38))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet version number"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packets.md") (range (start 20 35) (end 20 48)) (probe (position 20 35))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (kind specialization) (ordinal 0) (authored-target "Data Packet")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet")))))
  )
  (query (document "memory://snapshot/packets.md") (range (start 21 42) (end 21 70)) (probe (position 21 42))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (kind redefinition) (ordinal 0) (authored-target "Packets::packet data field")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field")))))
  )
  (query (document "memory://snapshot/packets.md") (range (start 22 49) (end 22 64)) (probe (position 22 49))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0) (authored-target "packet header")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")))))
  )
  (query (document "memory://snapshot/packets.md") (range (start 23 34) (end 23 42)) (probe (position 23 34))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::packet timestamp"))) (kind featureTyping) (ordinal 0) (authored-target "DateTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packets.md") (range (start 24 39) (end 24 45)) (probe (position 24 39))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::telemetry packet type"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packets.md") (range (start 27 41) (end 27 88)) (probe (position 27 41))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field"))) (kind redefinition) (ordinal 0) (authored-target "Packets::packet data field::user data field")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::user data field")))))
  )
  (query (document "memory://snapshot/packets.md") (range (start 29 27) (end 29 31)) (probe (position 29 27))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::temperature"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packets.md") (range (start 28 25) (end 28 33)) (probe (position 28 25))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::timestamp"))) (kind featureTyping) (ordinal 0) (authored-target "DateTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/packets.md") (range (start 7 48) (end 7 63)) (probe (position 7 48))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0) (authored-target "packet header")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")))))
  )
)
~~~
