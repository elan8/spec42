# META
~~~ini
description=SysML Example (Cause and Effect): CauseAndEffectExample
type=file
~~~
# SOURCE
~~~sysml
package CauseAndEffectExample {
	private import CauseAndEffect::*;
	
	part def Causer1;
	part def Causer2;
	part def Effected1;
	part def Effected2;
	
	#multicausation connection def MultiCauseEffect {
		end #cause cause1 : Causer1;
		end #cause cause2 : Causer2;
		end #effect effect1 : Effected1;
		end #effect effect2 : Effected2;
	}
	
	part causer1 : Causer1;
	part causer2 : Causer2;
	part effected1 : Effected1;
	part effected2 : Effected2;
	
	#multicausation connection : MultiCauseEffect connect
		( cause1 ::> causer1, cause2 ::> causer2,
		  effect1 ::> effected1, effect2 ::> effected2 );
		  
	#multicausation connect
		( cause1 ::> causer1, cause2 ::> causer2,
		  effect1 ::> effected1, effect2 ::> effected2 );

    occurrence a;
    item b;
    part c;
    action d;
    
	#multicausation connection {
		end #cause ::> a;
		end #cause ::> b;
		end #effect ::> c;
		end #effect ::> d;
	}
	
	#cause causeA ::> a;
	#cause causeB ::> b;
	#effect effectC ::> c;
	#effect effectD ::> d;
	
	#multicausation connect ( causeA, causeB, effectC, effectD );
	
	#causation connect a to c;
	#causation connect b to d {
		@CausationMetadata {
			isNecessary = true;
			probability = 0.1;
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
Hash,Ident,KwConnection,KwDef,Ident,OpenCurly,
KwEnd,Hash,Ident,Ident,Colon,Ident,Semicolon,
KwEnd,Hash,Ident,Ident,Colon,Ident,Semicolon,
KwEnd,Hash,Ident,Ident,Colon,Ident,Semicolon,
KwEnd,Hash,Ident,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
Hash,Ident,KwConnection,Colon,Ident,KwConnect,
OpenParen,Ident,ColonColonGt,Ident,Comma,Ident,ColonColonGt,Ident,Comma,
Ident,ColonColonGt,Ident,Comma,Ident,ColonColonGt,Ident,CloseParen,Semicolon,
Hash,Ident,KwConnect,
OpenParen,Ident,ColonColonGt,Ident,Comma,Ident,ColonColonGt,Ident,Comma,
Ident,ColonColonGt,Ident,Comma,Ident,ColonColonGt,Ident,CloseParen,Semicolon,
KwOccurrence,Ident,Semicolon,
KwItem,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwAction,Ident,Semicolon,
Hash,Ident,KwConnection,OpenCurly,
KwEnd,Hash,Ident,ColonColonGt,Ident,Semicolon,
KwEnd,Hash,Ident,ColonColonGt,Ident,Semicolon,
KwEnd,Hash,Ident,ColonColonGt,Ident,Semicolon,
KwEnd,Hash,Ident,ColonColonGt,Ident,Semicolon,
CloseCurly,
Hash,Ident,Ident,ColonColonGt,Ident,Semicolon,
Hash,Ident,Ident,ColonColonGt,Ident,Semicolon,
Hash,Ident,Ident,ColonColonGt,Ident,Semicolon,
Hash,Ident,Ident,ColonColonGt,Ident,Semicolon,
Hash,Ident,KwConnect,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
Hash,Ident,KwConnect,Ident,KwTo,Ident,Semicolon,
Hash,Ident,KwConnect,Ident,KwTo,Ident,OpenCurly,
At,Ident,OpenCurly,
Ident,Eq,KwTrue,Semicolon,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'CauseAndEffectExample'
    (import_decl private 'CauseAndEffect::*')
    (part_def 'Causer1')
    (part_def 'Causer2')
    (part_def 'Effected1')
    (part_def 'Effected2')
    (connection_def #'multicausation' 'MultiCauseEffect'
      (interface_end end #'cause' 'cause1' : 'Causer1')
      (interface_end end #'cause' 'cause2' : 'Causer2')
      (interface_end end #'effect' 'effect1' : 'Effected1')
      (interface_end end #'effect' 'effect2' : 'Effected2'))
    (part_usage 'causer1' : 'Causer1')
    (part_usage 'causer2' : 'Causer2')
    (part_usage 'effected1' : 'Effected1')
    (part_usage 'effected2' : 'Effected2')
    (connection_usage 'MultiCauseEffect')
    (connection_usage)
    (occurrence_usage 'a')
    (item_usage 'b')
    (part_usage 'c')
    (action_usage 'd')
    (malformed)
    (malformed)
    (extended_usage #'cause' 'causeA' references 'a')
    (extended_usage #'cause' 'causeB' references 'b')
    (extended_usage #'effect' 'effectC' references 'c')
    (extended_usage #'effect' 'effectD' references 'd')
    (connection_usage)
    (connection_usage
      (connector_end)
      (connector_end))
    (connection_usage
      (connector_end)
      (connector_end)
      (metadata_feature typed 'CausationMetadata'
        (feature_def 'isNecessary' value)
        (feature_def 'probability' value)))))
~~~
# FORMAT
~~~sysml
package CauseAndEffectExample {
	private import CauseAndEffect::*;
	
	part def Causer1;
	part def Causer2;
	part def Effected1;
	part def Effected2;
	
	#multicausation connection def MultiCauseEffect {
		end #cause cause1 : Causer1;
		end #cause cause2 : Causer2;
		end #effect effect1 : Effected1;
		end #effect effect2 : Effected2;
	}
	
	part causer1 : Causer1;
	part causer2 : Causer2;
	part effected1 : Effected1;
	part effected2 : Effected2;
	
	#multicausation connection : MultiCauseEffect connect
		( cause1 ::> causer1, cause2 ::> causer2,
		  effect1 ::> effected1, effect2 ::> effected2 );
		  
	#multicausation connect
		( cause1 ::> causer1, cause2 ::> causer2,
		  effect1 ::> effected1, effect2 ::> effected2 );

    occurrence a;
    item b;
    part c;
    action d;
    
	#multicausation connection {
		end #cause ::> a;
		end #cause ::> b;
		end #effect ::> c;
		end #effect ::> d;
	}
	
	#cause causeA ::> a;
	#cause causeB ::> b;
	#effect effectC ::> c;
	#effect effectD ::> d;
	
	#multicausation connect ( causeA, causeB, effectC, effectD );
	
	#causation connect a to c;
	#causation connect b to d {
		@CausationMetadata {
			isNecessary = true;
			probability = 0.1;
		}
	}
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
semantic.ambiguous_member 'malformed'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'CausationMetadata'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
semantic.ambiguous_member 'malformed'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'CausationMetadata'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "CauseAndEffectExample"))) (name "CauseAndEffectExample") (declared-name "CauseAndEffectExample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "CauseAndEffectExample::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1"))) (name "Causer1") (declared-name "Causer1") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2"))) (name "Causer2") (declared-name "Causer2") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1"))) (name "Effected1") (declared-name "Effected1") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2"))) (name "Effected2") (declared-name "Effected2") (declared))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect"))) (name "MultiCauseEffect") (declared-name "MultiCauseEffect")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (name "cause1") (declared-name "cause1") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (name "cause2") (declared-name "cause2") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (name "effect1") (declared-name "effect1") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (name "effect2") (declared-name "effect2") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect")))))
          )
        )
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_causation"))) (name "causation") (declared-name "causation"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_causation#metadata_keyword"))) (name "causation") (declared-name "causation"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_cause"))) (name "cause") (declared-name "cause"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_cause#metadata_keyword"))) (name "cause") (declared-name "cause"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef"))) (name "_connectionDef")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause"))) (name "#cause") (declared-name "#cause") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause#interface_end"))) (name "#cause") (declared-name "#cause") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect"))) (name "#effect") (declared-name "#effect") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect#interface_end"))) (name "#effect") (declared-name "#effect") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef")))))
          )
        )
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_effect"))) (name "effect") (declared-name "effect"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_effect#metadata_keyword"))) (name "effect") (declared-name "effect"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_multicausation"))) (name "multicausation") (declared-name "multicausation"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_multicausation#metadata_keyword"))) (name "multicausation") (declared-name "multicausation"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "CauseAndEffectExample::_multicausation#metadata_keyword2"))) (name "multicausation") (declared-name "multicausation"))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "CauseAndEffectExample::a"))) (name "a") (declared-name "a") (declared (properties (composite true) (reference false))))
        (element (kind "item def") (id (node (document "d0") (qualified-name "CauseAndEffectExample::b"))) (name "b") (declared-name "b"))
        (element (kind "part") (id (node (document "d0") (qualified-name "CauseAndEffectExample::c"))) (name "c") (declared-name "c") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "CauseAndEffectExample::causer1"))) (name "causer1") (declared-name "causer1") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "CauseAndEffectExample::causer2"))) (name "causer2") (declared-name "causer2") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "CauseAndEffectExample::connection"))) (name "connection") (declared-name "connection"))
        (element (kind "action") (id (node (document "d0") (qualified-name "CauseAndEffectExample::d"))) (name "d") (declared-name "d") (declared (properties (composite true) (reference false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "CauseAndEffectExample::effected1"))) (name "effected1") (declared-name "effected1") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "CauseAndEffectExample::effected2"))) (name "effected2") (declared-name "effected2") (declared (properties (composite true) (reference false) (ordered false))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::_causation"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::_causation#metadata_keyword"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::_cause"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::_cause#metadata_keyword"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::_effect"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::_effect#metadata_keyword"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::_multicausation"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::_multicausation#metadata_keyword"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::_multicausation#metadata_keyword2"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::a"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::b"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::a"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::c"))) (connect (source-expression "a") (target-expression "c") (container-prefix "CauseAndEffectExample")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::a"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::d"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::b"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::d"))) (connect (source-expression "b") (target-expression "d") (container-prefix "CauseAndEffectExample")))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::causer1"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::causer2"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::effected1"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CauseAndEffectExample::effected2"))) (to (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
