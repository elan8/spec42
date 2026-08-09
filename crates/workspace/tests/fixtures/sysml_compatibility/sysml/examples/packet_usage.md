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
        (element (kind "part") (id (node (document "d0") (qualified-name "Packet Usage::packet1"))) (name "packet1") (declared-name "packet1") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Packet Usage::packet2"))) (name "packet2") (declared-name "packet2") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Packet Usage::packet3"))) (name "packet3") (declared-name "packet3") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Packet Usage::packet3::special data field"))) (name "special data field") (declared-name "special data field") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
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
)
~~~
