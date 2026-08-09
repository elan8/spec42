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

    part packet1 : 'Thermal Data Packet';
    part packet2 : 'Thermal Data Packet';
    part packet3 : 'Thermal Data Packet' {
        attribute 'special data field' redefines 'packet data field' {
            attribute redefines 'user data field' {
                attribute 'special data' : Real;
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
(model
  (namespace
    (package 'Packet Usage'
      (namespace_import public -> 'Packets'[unresolved])
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (part_usage 'packet1' : 'Thermal Data Packet'[unresolved])
      (part_usage 'packet2' : 'Thermal Data Packet'[unresolved])
      (part_usage 'packet3' : 'Thermal Data Packet'[unresolved]
        (attribute_usage composite 'special data field' :>> 'packet data field'[unresolved]
          (attribute_usage composite :>> 'user data field'[unresolved]
            (attribute_usage composite 'special data' : 'Real'[unresolved])))))))
~~~
