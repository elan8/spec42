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
  (document "packet_usage.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 22))
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
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "50152335856237a1f718c0946355b12457b54dd67cae649487b2d894ca6d12fa") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Packet Usage"))) (kind "package") (name "Packet Usage") (declared-name "Packet Usage"))
    (element (id (node (document "d0") (qualified-name "Packet Usage::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Packet Usage"))) (authored (membership (kind Import) (visibility "public") (import (reference "Packets::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "Packet Usage"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet1"))) (kind "part") (name "packet1") (declared-name "packet1") (parent (node (document "d0") (qualified-name "Packet Usage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thermal Data Packet")))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet2"))) (kind "part") (name "packet2") (declared-name "packet2") (parent (node (document "d0") (qualified-name "Packet Usage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thermal Data Packet")))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet3"))) (kind "part") (name "packet3") (declared-name "packet3") (parent (node (document "d0") (qualified-name "Packet Usage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thermal Data Packet")))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet3::special data field"))) (kind "attribute") (name "special data field") (declared-name "special data field") (parent (node (document "d0") (qualified-name "Packet Usage::packet3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "packet data field")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Packet Usage::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Packets::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packet Usage::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packet Usage::packet1"))) (kind featureTyping) (ordinal 0)) (authored-target "Thermal Data Packet") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packet Usage::packet2"))) (kind featureTyping) (ordinal 0)) (authored-target "Thermal Data Packet") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packet Usage::packet3"))) (kind featureTyping) (ordinal 0)) (authored-target "Thermal Data Packet") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packet Usage::packet3::special data field"))) (kind redefinition) (ordinal 0)) (authored-target "packet data field") (outcome (status unresolved)))
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
    (query (range (start 1 15) (end 1 22)) (probe (position 1 15))
      (reference
        (source (document "d0") (qualified-name "Packet Usage::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Packets::*")
        (range (start 1 15) (end 1 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 34)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Packet Usage::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 2 16) (end 2 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 43) (end 7 62)) (probe (position 7 43))
      (reference
        (source (document "d0") (qualified-name "Packet Usage::packet3::special data field"))
        (kind redefinition) (ordinal 0) (authored-target "packet data field")
        (range (start 7 43) (end 7 62))
        (outcome (status unresolved))
      )
    )
    (query (range (start 4 15) (end 4 36)) (probe (position 4 15))
      (reference
        (source (document "d0") (qualified-name "Packet Usage::packet1"))
        (kind featureTyping) (ordinal 0) (authored-target "Thermal Data Packet")
        (range (start 4 15) (end 4 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 15) (end 5 36)) (probe (position 5 15))
      (reference
        (source (document "d0") (qualified-name "Packet Usage::packet2"))
        (kind featureTyping) (ordinal 0) (authored-target "Thermal Data Packet")
        (range (start 5 15) (end 5 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 15) (end 6 36)) (probe (position 6 15))
      (reference
        (source (document "d0") (qualified-name "Packet Usage::packet3"))
        (kind featureTyping) (ordinal 0) (authored-target "Thermal Data Packet")
        (range (start 6 15) (end 6 36))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
