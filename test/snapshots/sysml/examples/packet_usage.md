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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,Colon,UnrestrictedName,Semicolon,
KwPart,Ident,Colon,UnrestrictedName,Semicolon,
KwPart,Ident,Colon,UnrestrictedName,OpenCurly,
KwAttribute,UnrestrictedName,KwRedefines,UnrestrictedName,OpenCurly,
KwAttribute,KwRedefines,UnrestrictedName,OpenCurly,
KwAttribute,UnrestrictedName,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Packet Usage''
    (import_decl public 'Packets::*')
    (import_decl private 'ScalarValues::Real')
    (part_usage 'packet1' : ''Thermal Data Packet'')
    (part_usage 'packet2' : ''Thermal Data Packet'')
    (part_usage 'packet3' : ''Thermal Data Packet''
      (attribute_usage ''special data field'' :>> ''packet data field''
        (attribute_usage :>> ''user data field''
          (attribute_usage ''special data'' : 'Real'))))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Thermal Data Packet'
semantic.unresolved_name 'Thermal Data Packet'
semantic.unresolved_name 'Thermal Data Packet'
semantic.unresolved_name 'packet data field'
semantic.unresolved_name 'user data field'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Thermal Data Packet'
semantic.unresolved_name 'Thermal Data Packet'
semantic.unresolved_name 'Thermal Data Packet'
semantic.unresolved_name 'packet data field'
semantic.unresolved_name 'user data field'
semantic.unresolved_name 'Real'
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e18fab1cea07a0afbfc10c1e79f5da661899a24adccd62b00b7040d881ac11a4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Packet Usage"))) (kind "package") (name "Packet Usage") (declared-name "Packet Usage") (range (start (line 0) (character 0)) (end (line 0) (character 363))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 26))) (parent (node (document "d0") (qualified-name "Packet Usage"))) (authored (membership (kind Import) (visibility "public") (import (reference "Packets::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 15)) (end (line 1) (character 22))))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 2) (character 1)) (end (line 2) (character 35))) (parent (node (document "d0") (qualified-name "Packet Usage"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet1"))) (kind "part") (name "packet1") (declared-name "packet1") (range (start (line 4) (character 1)) (end (line 4) (character 37))) (parent (node (document "d0") (qualified-name "Packet Usage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thermal Data Packet") (range (start (line 4) (character 15)) (end (line 4) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet2"))) (kind "part") (name "packet2") (declared-name "packet2") (range (start (line 5) (character 1)) (end (line 5) (character 37))) (parent (node (document "d0") (qualified-name "Packet Usage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thermal Data Packet") (range (start (line 5) (character 15)) (end (line 5) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet3"))) (kind "part") (name "packet3") (declared-name "packet3") (range (start (line 6) (character 1)) (end (line 6) (character 193))) (parent (node (document "d0") (qualified-name "Packet Usage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thermal Data Packet") (range (start (line 6) (character 15)) (end (line 6) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "Packet Usage::packet3::special data field"))) (kind "attribute") (name "special data field") (declared-name "special data field") (range (start (line 7) (character 2)) (end (line 7) (character 151))) (parent (node (document "d0") (qualified-name "Packet Usage::packet3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "packet data field") (range (start (line 7) (character 43)) (end (line 7) (character 62)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Packet Usage::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Packets::*") (range (start (line 1) (character 15)) (end (line 1) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packet Usage::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 2) (character 16)) (end (line 2) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packet Usage::packet1"))) (kind featureTyping) (ordinal 0)) (authored-target "Thermal Data Packet") (range (start (line 4) (character 15)) (end (line 4) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packet Usage::packet2"))) (kind featureTyping) (ordinal 0)) (authored-target "Thermal Data Packet") (range (start (line 5) (character 15)) (end (line 5) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packet Usage::packet3"))) (kind featureTyping) (ordinal 0)) (authored-target "Thermal Data Packet") (range (start (line 6) (character 15)) (end (line 6) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Packet Usage::packet3::special data field"))) (kind redefinition) (ordinal 0)) (authored-target "packet data field") (range (start (line 7) (character 43)) (end (line 7) (character 62))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
