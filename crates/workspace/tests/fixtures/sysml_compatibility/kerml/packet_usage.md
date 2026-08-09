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
# TOKENS
~~~zig
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPackage,UnrestrictedName,OpenCurly,
KwFeature,Ident,Colon,UnrestrictedName,Semicolon,
KwFeature,Ident,Colon,UnrestrictedName,Semicolon,
KwFeature,Ident,Colon,UnrestrictedName,OpenCurly,
KwFeature,UnrestrictedName,KwRedefines,UnrestrictedName,OpenCurly,
KwFeature,ColonGtGt,UnrestrictedName,OpenCurly,
KwFeature,UnrestrictedName,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
EndOfFile,
~~~
# AST
~~~
(root
  (import_decl private 'Packets::*')
  (import_decl private 'ScalarValues::Real')
  (package_def ''Packet Usage''
    (feature_def 'packet1' : ''Thermal Data Packet'')
    (feature_def 'packet2' : ''Thermal Data Packet'')
    (feature_def 'packet3' : ''Thermal Data Packet''
      (feature_def ''special data field'' :>> ''packet data field''
        (feature_def :>> ''user data field''
          (feature_def ''special data'' : 'Real'))))))
~~~
# FORMAT
~~~sysml
private import Packets::*;
private import ScalarValues::Real;
package 'Packet Usage' {
    feature packet1 : 'Thermal Data Packet';
    feature packet2 : 'Thermal Data Packet';
    feature packet3 : 'Thermal Data Packet' {
        feature 'special data field' redefines 'packet data field' {
            feature :>> 'user data field' {
                feature 'special data' : Real;
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
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Packet Usage::packet1"))) (name "packet1") (declared-name "packet1"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Packet Usage::packet2"))) (name "packet2") (declared-name "packet2"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Packet Usage::packet3"))) (name "packet3") (declared-name "packet3"))
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
