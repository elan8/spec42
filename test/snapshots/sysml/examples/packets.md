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
    (element (id (node (document "d0") (qualified-name "Packets"))) (kind "package") (name "Packets") (declared-name "Packets"))
    (element (id (node (document "d0") (qualified-name "Packets::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Packets"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Packets::Data Packet"))) (kind "part def") (name "Data Packet") (declared-name "Data Packet") (parent (node (document "d0") (qualified-name "Packets"))))
    (element (id (node (document "d0") (qualified-name "Packets::Data Packet::packet data field"))) (kind "attribute") (name "packet data field") (declared-name "packet data field") (parent (node (document "d0") (qualified-name "Packets::Data Packet"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "packet data field")))))
    (element (id (node (document "d0") (qualified-name "Packets::Data Packet::packet primary header"))) (kind "attribute") (name "packet primary header") (declared-name "packet primary header") (parent (node (document "d0") (qualified-name "Packets::Data Packet"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "packet header")))))
    (element (id (node (document "d0") (qualified-name "Packets::DateTime"))) (kind "import") (name "DateTime") (declared-name "DateTime") (parent (node (document "d0") (qualified-name "Packets"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::DateTime") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Packets::Thermal Data Packet"))) (kind "part def") (name "Thermal Data Packet") (declared-name "Thermal Data Packet") (parent (node (document "d0") (qualified-name "Packets"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Data Packet")))))
    (element (id (node (document "d0") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (kind "attribute") (name "packet data field") (declared-name "packet data field") (parent (node (document "d0") (qualified-name "Packets::Thermal Data Packet"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Packets::packet data field")))))
    (element (id (node (document "d0") (qualified-name "Packets::packet data field"))) (kind "attribute def") (name "packet data field") (declared-name "packet data field") (parent (node (document "d0") (qualified-name "Packets"))))
    (element (id (node (document "d0") (qualified-name "Packets::packet data field::packet secondary header"))) (kind "attribute") (name "packet secondary header") (declared-name "packet secondary header") (parent (node (document "d0") (qualified-name "Packets::packet data field"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "packet header")))))
    (element (id (node (document "d0") (qualified-name "Packets::packet data field::user data field"))) (kind "attribute") (name "user data field") (declared-name "user data field") (parent (node (document "d0") (qualified-name "Packets::packet data field"))))
    (element (id (node (document "d0") (qualified-name "Packets::packet header"))) (kind "attribute def") (name "packet header") (declared-name "packet header") (parent (node (document "d0") (qualified-name "Packets"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Packets::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packets::Data Packet::packet data field"))) (kind redefinition) (ordinal 0)) (authored-target "packet data field") (outcome (status resolved) (target (node (document "d0") (qualified-name "Packets::Data Packet::packet data field")))))
    (reference (id (source (node (document "d0") (qualified-name "Packets::Data Packet::packet primary header"))) (kind redefinition) (ordinal 0)) (authored-target "packet header") (outcome (status resolved) (target (node (document "d0") (qualified-name "Packets::packet header")))))
    (reference (id (source (node (document "d0") (qualified-name "Packets::DateTime"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::DateTime") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packets::Thermal Data Packet"))) (kind specialization) (ordinal 0)) (authored-target "Data Packet") (outcome (status resolved) (target (node (document "d0") (qualified-name "Packets::Data Packet")))))
    (reference (id (source (node (document "d0") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (kind redefinition) (ordinal 0)) (authored-target "Packets::packet data field") (outcome (status resolved) (target (node (document "d0") (qualified-name "Packets::packet data field")))))
    (reference (id (source (node (document "d0") (qualified-name "Packets::packet data field::packet secondary header"))) (kind redefinition) (ordinal 0)) (authored-target "packet header") (outcome (status resolved) (target (node (document "d0") (qualified-name "Packets::packet header")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Packets::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 35) (end 20 48)) (probe (position 20 35))
      (reference
        (source (document "d0") (qualified-name "Packets::Thermal Data Packet"))
        (kind specialization) (ordinal 0) (authored-target "Data Packet")
        (range (start 20 35) (end 20 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Packets::Data Packet") (range (start 11 1) (end 11 276)))
        )
      )
    )
    (query (range (start 2 16) (end 2 30)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Packets::DateTime"))
        (kind membershipImport) (ordinal 0) (authored-target "Time::DateTime")
        (range (start 2 16) (end 2 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 48) (end 7 63)) (probe (position 7 48))
      (reference
        (source (document "d0") (qualified-name "Packets::packet data field::packet secondary header"))
        (kind redefinition) (ordinal 0) (authored-target "packet header")
        (range (start 7 48) (end 7 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Packets::packet header") (range (start 4 1) (end 4 30)))
        )
      )
    )
    (query (range (start 12 46) (end 12 61)) (probe (position 12 46))
      (reference
        (source (document "d0") (qualified-name "Packets::Data Packet::packet primary header"))
        (kind redefinition) (ordinal 0) (authored-target "packet header")
        (range (start 12 46) (end 12 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Packets::packet header") (range (start 4 1) (end 4 30)))
        )
      )
    )
    (query (range (start 17 22) (end 17 41)) (probe (position 17 22))
      (reference
        (source (document "d0") (qualified-name "Packets::Data Packet::packet data field"))
        (kind redefinition) (ordinal 0) (authored-target "packet data field")
        (range (start 17 22) (end 17 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Packets::Data Packet::packet data field") (range (start 17 2) (end 17 42)))
        )
      )
    )
    (query (range (start 21 42) (end 21 70)) (probe (position 21 42))
      (reference
        (source (document "d0") (qualified-name "Packets::Thermal Data Packet::packet data field"))
        (kind redefinition) (ordinal 0) (authored-target "Packets::packet data field")
        (range (start 21 42) (end 21 70))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Packets::packet data field") (range (start 6 1) (end 6 132)))
        )
      )
    )
  )
)
~~~
