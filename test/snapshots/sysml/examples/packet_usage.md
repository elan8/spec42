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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Packet Usage"))) (name "Packet Usage") (declared-name "Packet Usage")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Packet Usage::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Packet Usage::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "part") (id (node (document "d0") (qualified-name "Packet Usage::packet1"))) (name "packet1") (declared-name "packet1") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Packet Usage::packet2"))) (name "packet2") (declared-name "packet2") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Packet Usage::packet3"))) (name "packet3") (declared-name "packet3") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Packet Usage::packet3::special data field"))) (name "special data field") (declared-name "special data field") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
          )
        )
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Packet Usage::packet1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Packet Usage::packet2"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Packet Usage::packet3"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Packet Usage::packet3::special data field"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/packet_usage.md"
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 7 2) (end 7 151))
      )
    )
  )
)
~~~
