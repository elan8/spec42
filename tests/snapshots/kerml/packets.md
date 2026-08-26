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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 0 15) (end 0 30))
      )
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 36) (end 13 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 36) (end 14 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 33) (end 15 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 32) (end 23 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 37) (end 24 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 23) (end 28 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 25) (end 29 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2cc65cd9d54e1a2b7ea0f95816ad1053c065af4d476a2d3aefacda1e9a8d356a") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/packets.md") (path (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/packets.md") (path (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::DateTime") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind class-def) (name "Data Packet")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "packet data field")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "packet header")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet data length"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet identification"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet version number"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Data Packet")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Packets::packet data field")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "packet header")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::packet timestamp"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DateTime")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::telemetry packet type"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Packets::packet data field::user data field")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::temperature"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::timestamp"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DateTime")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "packet header")))))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::user data field"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/packets.md") (path (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (path (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind class-def) (name "Data Packet")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
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
    (relationship (kind redefinition) (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind class-def) (name "Data Packet")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind class-def) (name "Data Packet")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::user data field"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind class-def) (name "Data Packet")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet data length"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet identification"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet version number"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::packet timestamp"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::telemetry packet type"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::temperature"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::timestamp"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::user data field"))) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet")))
      (subtype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind class-def) (name "Data Packet")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet")))
      (supertype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet")))
      (supertype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet data length")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header")))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet identification")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header")))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet version number")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header")))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet")))
      (supertype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet")))
      (supertype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field")))
      (supertype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::packet timestamp")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header")))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::telemetry packet type")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header")))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field")))
      (supertype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::user data field")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::temperature")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field")))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::timestamp")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field")))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field")))
      (subtype (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind class-def) (name "Data Packet")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field")))
      (supertype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::user data field")))
      (featured-by (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field")))
      (subtype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")))
      (subtype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header")) (scopes any feature))
      (subtype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header")) (scopes any feature))
      (subtype (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/packets.md") (range (start 0 15) (end 0 30)) (probe (position 0 15))
    (reference (id (source (node (document "memory://snapshot/packets.md") (path (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 1 15) (end 1 29)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/packets.md") (path (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 17 20) (end 17 39)) (probe (position 17 20))
    (reference (id (source (node (document "memory://snapshot/packets.md") (path (named (kind package) (name "Packets")) (named (kind class-def) (name "Data Packet")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "packet data field")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field")))))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 12 44) (end 12 59)) (probe (position 12 44))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header"))) (kind redefinition) (ordinal 0) (authored-target "packet header")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")))))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 15 33) (end 15 40)) (probe (position 15 33))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet data length"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 14 36) (end 14 42)) (probe (position 14 36))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet identification"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 13 36) (end 13 43)) (probe (position 13 36))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet::packet primary header::packet version number"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 20 41) (end 20 54)) (probe (position 20 41))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet"))) (kind specialization) (ordinal 0) (authored-target "Data Packet")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Data Packet")))))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 21 40) (end 21 68)) (probe (position 21 40))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (kind redefinition) (ordinal 0) (authored-target "Packets::packet data field")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field")))))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 22 47) (end 22 62)) (probe (position 22 47))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0) (authored-target "packet header")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")))))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 23 32) (end 23 40)) (probe (position 23 32))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::packet timestamp"))) (kind featureTyping) (ordinal 0) (authored-target "DateTime")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 24 37) (end 24 43)) (probe (position 24 37))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::packet secondary header::telemetry packet type"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 27 39) (end 27 86)) (probe (position 27 39))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field"))) (kind redefinition) (ordinal 0) (authored-target "Packets::packet data field::user data field")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::user data field")))))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 29 25) (end 29 29)) (probe (position 29 25))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::temperature"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 28 23) (end 28 31)) (probe (position 28 23))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::Thermal Data Packet::packet data field::user data field::timestamp"))) (kind featureTyping) (ordinal 0) (authored-target "DateTime")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/packets.md") (range (start 7 46) (end 7 61)) (probe (position 7 46))
    (reference (id (source (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0) (authored-target "packet header")
      (outcome (status resolved) (target (node (document "memory://snapshot/packets.md") (qualified-name "Packets::packet header")))))
    )
  )
)
~~~
