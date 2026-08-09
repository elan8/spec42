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
            attribute 'packet version number' : Integer;
            attribute 'packet identification' : String;
            attribute 'packet data length' : Integer;
        }
        attribute redefines 'packet data field';
    }

    part def 'Thermal Data Packet' :> 'Data Packet' {
        attribute 'packet data field' redefines Packets::'packet data field' {
            attribute 'packet secondary header' redefines 'packet header' {
                attribute 'packet timestamp' : DateTime;
                attribute 'telemetry packet type' : String;
            }

            attribute 'user data field' redefines Packets::'packet data field'::'user data field' {
                attribute timestamp : DateTime;
                attribute temperature : Real;
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
(model
  (namespace
    (package 'Packets'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (membership_import private -> 'Time::DateTime'[unresolved])
      (attribute_usage 'packet header')
      (attribute_usage 'packet data field'
        (attribute_usage composite 'packet secondary header' :>> 'Packets::packet header'[attribute_usage])
        (attribute_usage composite 'user data field'))
      (part_def 'Data Packet'
        (attribute_usage composite 'packet primary header' :>> 'Packets::packet header'[attribute_usage]
          (attribute_usage composite 'packet version number' : 'Integer'[unresolved])
          (attribute_usage composite 'packet identification' : 'String'[unresolved])
          (attribute_usage composite 'packet data length' : 'Integer'[unresolved]))
        (attribute_usage composite :>> 'Packets::packet data field'[attribute_usage]))
      (part_def 'Thermal Data Packet' :> 'Packets::Data Packet'[part_def]
        (attribute_usage composite 'packet data field' :>> 'Packets::packet data field'[attribute_usage]
          (attribute_usage composite 'packet secondary header' :>> 'Packets::packet header'[attribute_usage]
            (attribute_usage composite 'packet timestamp' : 'DateTime'[unresolved])
            (attribute_usage composite 'telemetry packet type' : 'String'[unresolved]))
          (attribute_usage composite 'user data field' :>> 'Packets::packet data field::user data field'[attribute_usage]
            (attribute_usage composite 'timestamp' : 'DateTime'[unresolved])
            (attribute_usage composite 'temperature' : 'Real'[unresolved])))))))
~~~
