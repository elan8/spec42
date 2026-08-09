# META
~~~ini
description=KerML Packet: Packets
type=file
~~~
# SOURCE
~~~kerml
private import ScalarValues::*;
private import Time::DateTime;
package Packets {
	
	feature 'packet header' { }
	
	feature 'packet data field' {	
		feature 'packet secondary header' redefines 'packet header';
		feature 'user data field';
	}
	
	class 'Data Packet' { 
		feature 'packet primary header' redefines 'packet header' {
			feature 'packet version number': Integer;
			feature 'packet identification': String;
			feature 'packet data length': Integer;
		}
		feature redefines 'packet data field';
	}
	
	class 'Thermal Data Packet' specializes 'Data Packet' {
		feature 'packet data field' redefines Packets::'packet data field'{
			feature 'packet secondary header' redefines 'packet header' {
				feature 'packet timestamp': DateTime;
				feature 'telemetry packet type': String;
			}
			
			feature 'user data field' redefines Packets::'packet data field'::'user data field' {
				feature timestamp: DateTime;
				feature temperature: Real;
			}
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPackage,Ident,OpenCurly,
KwFeature,UnrestrictedName,OpenCurly,CloseCurly,
KwFeature,UnrestrictedName,OpenCurly,
KwFeature,UnrestrictedName,KwRedefines,UnrestrictedName,Semicolon,
KwFeature,UnrestrictedName,Semicolon,
CloseCurly,
KwClass,UnrestrictedName,OpenCurly,
KwFeature,UnrestrictedName,KwRedefines,UnrestrictedName,OpenCurly,
KwFeature,UnrestrictedName,Colon,Ident,Semicolon,
KwFeature,UnrestrictedName,Colon,Ident,Semicolon,
KwFeature,UnrestrictedName,Colon,Ident,Semicolon,
CloseCurly,
KwFeature,KwRedefines,UnrestrictedName,Semicolon,
CloseCurly,
KwClass,UnrestrictedName,KwSpecializes,UnrestrictedName,OpenCurly,
KwFeature,UnrestrictedName,KwRedefines,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwFeature,UnrestrictedName,KwRedefines,UnrestrictedName,OpenCurly,
KwFeature,UnrestrictedName,Colon,Ident,Semicolon,
KwFeature,UnrestrictedName,Colon,Ident,Semicolon,
CloseCurly,
KwFeature,UnrestrictedName,KwRedefines,Ident,ColonColon,UnrestrictedName,ColonColon,UnrestrictedName,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (import_decl private 'ScalarValues::*')
  (import_decl private 'Time::DateTime')
  (package_def 'Packets'
    (feature_def ''packet header'')
    (feature_def ''packet data field''
      (feature_def ''packet secondary header'' :>> ''packet header'')
      (feature_def ''user data field''))
    (class_def ''Data Packet''
      (feature_def ''packet primary header'' :>> ''packet header''
        (feature_def ''packet version number'' : 'Integer')
        (feature_def ''packet identification'' : 'String')
        (feature_def ''packet data length'' : 'Integer'))
      (feature_def :>> ''packet data field''))
    (class_def ''Thermal Data Packet'' :> ''Data Packet''
      (feature_def ''packet data field'' :>> 'Packets::'packet data field''
        (feature_def ''packet secondary header'' :>> ''packet header''
          (feature_def ''packet timestamp'' : 'DateTime')
          (feature_def ''telemetry packet type'' : 'String'))
        (feature_def ''user data field'' :>> 'Packets::'packet data field'::'user data field''
          (feature_def 'timestamp' : 'DateTime')
          (feature_def 'temperature' : 'Real'))))))
~~~
# FORMAT
~~~sysml
private import ScalarValues::*;
private import Time::DateTime;
package Packets {
	
	feature 'packet header' { }
	
	feature 'packet data field' {	
		feature 'packet secondary header' redefines 'packet header';
		feature 'user data field';
	}
	
	class 'Data Packet' { 
		feature 'packet primary header' redefines 'packet header' {
			feature 'packet version number': Integer;
			feature 'packet identification': String;
			feature 'packet data length': Integer;
		}
		feature redefines 'packet data field';
	}
	
	class 'Thermal Data Packet' specializes 'Data Packet' {
		feature 'packet data field' redefines Packets::'packet data field'{
			feature 'packet secondary header' redefines 'packet header' {
				feature 'packet timestamp': DateTime;
				feature 'telemetry packet type': String;
			}
			
			feature 'user data field' redefines Packets::'packet data field'::'user data field' {
				feature timestamp: DateTime;
				feature temperature: Real;
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
    (element (kind "import") (id (node (document "d0") (qualified-name "*"))) (name "*") (declared-name "*"))
    (element (kind "import") (id (node (document "d0") (qualified-name "DateTime"))) (name "DateTime") (declared-name "DateTime"))
    (element (kind "package") (id (node (document "d0") (qualified-name "Packets"))) (name "Packets") (declared-name "Packets")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Packets::Data"))) (name "Data") (declared-name "Data"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Packets::Packet"))) (name "Packet") (declared-name "Packet"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Packets::data"))) (name "data") (declared-name "data"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Packets::header"))) (name "header") (declared-name "header"))
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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/packets.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 0) (end 0 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 0) (end 1 30))
      )
    )
  )
)
~~~
