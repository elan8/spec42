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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "packets.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 15) (end 0 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 29))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1b0a3aec22c5ca757c2a4d135c42ac784a6b0231d4d3086fd76e47bb12bca31c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 0) (character 0)) (end (line 0) (character 31))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 0) (character 15)) (end (line 0) (character 27))))))
    (element (id (node (document "d0") (qualified-name "DateTime"))) (kind "import") (name "DateTime") (declared-name "DateTime") (range (start (line 1) (character 0)) (end (line 1) (character 30))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::DateTime") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 15)) (end (line 1) (character 29))))))
    (element (id (node (document "d0") (qualified-name "Packets"))) (kind "package") (name "Packets") (declared-name "Packets") (range (start (line 2) (character 0)) (end (line 2) (character 903))))
    (element (id (node (document "d0") (qualified-name "Packets::Data"))) (kind "classifier decl") (name "Data") (declared-name "Data") (range (start (line 20) (character 1)) (end (line 20) (character 452))) (parent (node (document "d0") (qualified-name "Packets"))))
    (element (id (node (document "d0") (qualified-name "Packets::Packet"))) (kind "classifier decl") (name "Packet") (declared-name "Packet") (range (start (line 11) (character 1)) (end (line 11) (character 264))) (parent (node (document "d0") (qualified-name "Packets"))))
    (element (id (node (document "d0") (qualified-name "Packets::data"))) (kind "feature decl") (name "data") (declared-name "data") (range (start (line 6) (character 1)) (end (line 6) (character 126))) (parent (node (document "d0") (qualified-name "Packets"))))
    (element (id (node (document "d0") (qualified-name "Packets::header"))) (kind "feature decl") (name "header") (declared-name "header") (range (start (line 4) (character 1)) (end (line 4) (character 28))) (parent (node (document "d0") (qualified-name "Packets"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 0) (character 15)) (end (line 0) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DateTime"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::DateTime") (range (start (line 1) (character 15)) (end (line 1) (character 29))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
