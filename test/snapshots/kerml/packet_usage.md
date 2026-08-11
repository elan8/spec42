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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "77990fc602b1a57d1e6709a71b2eecafdfdb633c6b0aa15863475ba9f18143b3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "*"))) (kind "import") (name "*") (declared-name "*") (authored (membership (kind Import) (visibility "private") (import (reference "Packets::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Packet Usage"))) (kind "package") (name "Packet Usage") (declared-name "Packet Usage"))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet1"))) (kind "feature decl") (name "packet1") (declared-name "packet1") (parent (node (document "d0") (qualified-name "Packet Usage"))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet2"))) (kind "feature decl") (name "packet2") (declared-name "packet2") (parent (node (document "d0") (qualified-name "Packet Usage"))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet3"))) (kind "feature decl") (name "packet3") (declared-name "packet3") (parent (node (document "d0") (qualified-name "Packet Usage"))))
    (element (id (node (document "d0") (qualified-name "Real"))) (kind "import") (name "Real") (declared-name "Real") (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Packets::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
  (document "d0"
    (query (range (start 0 15) (end 0 22)) (probe (position 0 15))
      (reference
        (source (document "d0") (qualified-name "*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Packets::*")
        (range (start 0 15) (end 0 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 15) (end 1 33)) (probe (position 1 15))
      (reference
        (source (document "d0") (qualified-name "Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 15) (end 1 33))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
