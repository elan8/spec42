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
  (document "packets.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 30))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "52317ea22aa8e8bba5df8839ee8e9650c2fb83e0ce3a3e70ad626ee4654abc23") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Packets"))) (kind "package") (name "Packets") (declared-name "Packets") (range (start (line 0) (character 0)) (end (line 0) (character 996))))
    (element (id (node (document "d0") (qualified-name "Packets::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "Packets"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Packets::Data Packet"))) (kind "part def") (name "Data Packet") (declared-name "Data Packet") (range (start (line 11) (character 1)) (end (line 11) (character 276))) (parent (node (document "d0") (qualified-name "Packets"))))
    (element (id (node (document "d0") (qualified-name "Packets::Data Packet::packet data field"))) (kind "attribute") (name "packet data field") (declared-name "packet data field") (range (start (line 17) (character 2)) (end (line 17) (character 42))) (parent (node (document "d0") (qualified-name "Packets::Data Packet"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "packet data field") (range (start (line 17) (character 22)) (end (line 17) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "Packets::Data Packet::packet primary header"))) (kind "attribute") (name "packet primary header") (declared-name "packet primary header") (range (start (line 12) (character 2)) (end (line 12) (character 204))) (parent (node (document "d0") (qualified-name "Packets::Data Packet"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "packet header") (range (start (line 12) (character 46)) (end (line 12) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "Packets::DateTime"))) (kind "import") (name "DateTime") (declared-name "DateTime") (range (start (line 2) (character 1)) (end (line 2) (character 31))) (parent (node (document "d0") (qualified-name "Packets"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::DateTime") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Packets::Thermal Data Packet"))) (kind "part def") (name "Thermal Data Packet") (declared-name "Thermal Data Packet") (range (start (line 20) (character 1)) (end (line 20) (character 460))) (parent (node (document "d0") (qualified-name "Packets"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Data Packet") (range (start (line 20) (character 35)) (end (line 20) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (kind "attribute") (name "packet data field") (declared-name "packet data field") (range (start (line 21) (character 2)) (end (line 21) (character 406))) (parent (node (document "d0") (qualified-name "Packets::Thermal Data Packet"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Packets::packet data field") (range (start (line 21) (character 42)) (end (line 21) (character 70)))))))
    (element (id (node (document "d0") (qualified-name "Packets::packet data field"))) (kind "attribute def") (name "packet data field") (declared-name "packet data field") (range (start (line 6) (character 1)) (end (line 6) (character 132))) (parent (node (document "d0") (qualified-name "Packets"))))
    (element (id (node (document "d0") (qualified-name "Packets::packet data field::packet secondary header"))) (kind "attribute") (name "packet secondary header") (declared-name "packet secondary header") (range (start (line 7) (character 2)) (end (line 7) (character 64))) (parent (node (document "d0") (qualified-name "Packets::packet data field"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "packet header") (range (start (line 7) (character 48)) (end (line 7) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "Packets::packet data field::user data field"))) (kind "attribute") (name "user data field") (declared-name "user data field") (range (start (line 8) (character 2)) (end (line 8) (character 30))) (parent (node (document "d0") (qualified-name "Packets::packet data field"))))
    (element (id (node (document "d0") (qualified-name "Packets::packet header"))) (kind "attribute def") (name "packet header") (declared-name "packet header") (range (start (line 4) (character 1)) (end (line 4) (character 30))) (parent (node (document "d0") (qualified-name "Packets"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Packets::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packets::Data Packet::packet data field"))) (kind redefinition) (ordinal 0)) (authored-target "packet data field") (range (start (line 17) (character 22)) (end (line 17) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Packets::Data Packet::packet data field")))))
    (reference (id (source (node (document "d0") (qualified-name "Packets::Data Packet::packet primary header"))) (kind redefinition) (ordinal 0)) (authored-target "packet header") (range (start (line 12) (character 46)) (end (line 12) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Packets::packet header")))))
    (reference (id (source (node (document "d0") (qualified-name "Packets::DateTime"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::DateTime") (range (start (line 2) (character 16)) (end (line 2) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packets::Thermal Data Packet"))) (kind specialization) (ordinal 0)) (authored-target "Data Packet") (range (start (line 20) (character 35)) (end (line 20) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Packets::Data Packet")))))
    (reference (id (source (node (document "d0") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (kind redefinition) (ordinal 0)) (authored-target "Packets::packet data field") (range (start (line 21) (character 42)) (end (line 21) (character 70))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Packets::packet data field")))))
    (reference (id (source (node (document "d0") (qualified-name "Packets::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0)) (authored-target "packet header") (range (start (line 7) (character 48)) (end (line 7) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Packets::packet header")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Packets::Data Packet::packet data field"))) (target (node (document "d0") (qualified-name "Packets::Data Packet::packet data field"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Packets::Data Packet::packet data field"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Packets::Data Packet::packet primary header"))) (target (node (document "d0") (qualified-name "Packets::packet header"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Packets::Data Packet::packet primary header"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Packets::Thermal Data Packet"))) (target (node (document "d0") (qualified-name "Packets::Data Packet"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Packets::Thermal Data Packet"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (target (node (document "d0") (qualified-name "Packets::packet data field"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Packets::packet data field::packet secondary header"))) (target (node (document "d0") (qualified-name "Packets::packet header"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Packets::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
