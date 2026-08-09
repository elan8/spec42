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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,UnrestrictedName,OpenCurly,CloseCurly,
KwAttribute,UnrestrictedName,OpenCurly,
KwAttribute,UnrestrictedName,KwRedefines,UnrestrictedName,Semicolon,
KwAttribute,UnrestrictedName,Semicolon,
CloseCurly,
KwPart,KwDef,UnrestrictedName,OpenCurly,
KwAttribute,UnrestrictedName,KwRedefines,UnrestrictedName,OpenCurly,
KwAttribute,UnrestrictedName,Colon,Ident,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwRedefines,UnrestrictedName,Semicolon,
CloseCurly,
KwPart,KwDef,UnrestrictedName,ColonGt,UnrestrictedName,OpenCurly,
KwAttribute,UnrestrictedName,KwRedefines,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwAttribute,UnrestrictedName,KwRedefines,UnrestrictedName,OpenCurly,
KwAttribute,UnrestrictedName,Colon,Ident,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,UnrestrictedName,KwRedefines,Ident,ColonColon,UnrestrictedName,ColonColon,UnrestrictedName,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Packets'
    (import_decl private 'ScalarValues::*')
    (import_decl private 'Time::DateTime')
    (attribute_usage ''packet header'')
    (attribute_usage ''packet data field''
      (attribute_usage ''packet secondary header'' :>> ''packet header'')
      (attribute_usage ''user data field''))
    (part_def ''Data Packet''
      (attribute_usage ''packet primary header'' :>> ''packet header''
        (attribute_usage ''packet version number'' : 'Integer')
        (attribute_usage ''packet identification'' : 'String')
        (attribute_usage ''packet data length'' : 'Integer'))
      (attribute_usage :>> ''packet data field''))
    (part_def ''Thermal Data Packet'' :> ''Data Packet''
      (attribute_usage ''packet data field'' :>> 'Packets::'packet data field''
        (attribute_usage ''packet secondary header'' :>> ''packet header''
          (attribute_usage ''packet timestamp'' : 'DateTime')
          (attribute_usage ''telemetry packet type'' : 'String'))
        (attribute_usage ''user data field'' :>> 'Packets::'packet data field'::'user data field''
          (attribute_usage 'timestamp' : 'DateTime')
          (attribute_usage 'temperature' : 'Real'))))))
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
# EXPECTED
~~~
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'DateTime'
semantic.unresolved_name 'String'
semantic.unresolved_name 'DateTime'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'DateTime'
semantic.unresolved_name 'String'
semantic.unresolved_name 'DateTime'
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Packets"))) (name "Packets") (declared-name "Packets")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Packets::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Packets::Data Packet"))) (name "Data Packet") (declared-name "Data Packet") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Packets::Data Packet::packet data field"))) (name "packet data field") (declared-name "packet data field") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Packets::Data Packet")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Packets::Data Packet::packet primary header"))) (name "packet primary header") (declared-name "packet primary header") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Packets::Data Packet")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Packets::DateTime"))) (name "DateTime") (declared-name "DateTime"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Packets::Thermal Data Packet"))) (name "Thermal Data Packet") (declared-name "Thermal Data Packet") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (name "packet data field") (declared-name "packet data field") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Packets::Thermal Data Packet")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Packets::packet data field"))) (name "packet data field") (declared-name "packet data field") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Packets::packet data field::packet secondary header"))) (name "packet secondary header") (declared-name "packet secondary header") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Packets::packet data field")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Packets::packet data field::user data field"))) (name "user data field") (declared-name "user data field") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Packets::packet data field")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Packets::packet header"))) (name "packet header") (declared-name "packet header") (declared (properties (ordered false) (unique true))))
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Packets::Thermal Data Packet::packet data field"))) (to (node (document "d0") (qualified-name "Packets::packet data field"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Packets::Thermal Data Packet"))) (to (node (document "d0") (qualified-name "Packets::Data Packet"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/packets.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 7 2) (end 7 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 12 2) (end 12 204))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 17 2) (end 17 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 21 2) (end 21 406))
      )
    )
  )
)
~~~
