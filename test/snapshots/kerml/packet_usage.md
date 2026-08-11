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
  (document "packet_usage.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 15) (end 0 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 33))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "77990fc602b1a57d1e6709a71b2eecafdfdb633c6b0aa15863475ba9f18143b3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 0) (character 0)) (end (line 0) (character 26))) (authored (membership (kind Import) (visibility "private") (import (reference "Packets::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 0) (character 15)) (end (line 0) (character 22))))))
    (element (id (node (document "d0") (qualified-name "Packet Usage"))) (kind "package") (name "Packet Usage") (declared-name "Packet Usage") (range (start (line 2) (character 0)) (end (line 2) (character 297))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet1"))) (kind "feature decl") (name "packet1") (declared-name "packet1") (range (start (line 4) (character 1)) (end (line 4) (character 40))) (parent (node (document "d0") (qualified-name "Packet Usage"))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet2"))) (kind "feature decl") (name "packet2") (declared-name "packet2") (range (start (line 5) (character 1)) (end (line 5) (character 40))) (parent (node (document "d0") (qualified-name "Packet Usage"))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet3"))) (kind "feature decl") (name "packet3") (declared-name "packet3") (range (start (line 6) (character 1)) (end (line 6) (character 184))) (parent (node (document "d0") (qualified-name "Packet Usage"))))
    (element (id (node (document "d0") (qualified-name "Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 0)) (end (line 1) (character 34))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 15)) (end (line 1) (character 33))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Packets::*") (range (start (line 0) (character 15)) (end (line 0) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 15)) (end (line 1) (character 33))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
