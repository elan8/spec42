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

    #multicausation
    connection {
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
(model
  (namespace
    (package 'CauseAndEffectExample'
      (namespace_import private -> 'CauseAndEffect'[unresolved])
      (part_def 'Causer1')
      (part_def 'Causer2')
      (part_def 'Effected1')
      (part_def 'Effected2')
      (connection_def 'MultiCauseEffect'
        (port_usage end 'cause1' : 'CauseAndEffectExample::Causer1'[part_def])
        (port_usage end 'cause2' : 'CauseAndEffectExample::Causer2'[part_def])
        (port_usage end 'effect1' : 'CauseAndEffectExample::Effected1'[part_def])
        (port_usage end 'effect2' : 'CauseAndEffectExample::Effected2'[part_def]))
      (part_usage 'causer1' : 'CauseAndEffectExample::Causer1'[part_def])
      (part_usage 'causer2' : 'CauseAndEffectExample::Causer2'[part_def])
      (part_usage 'effected1' : 'CauseAndEffectExample::Effected1'[part_def])
      (part_usage 'effected2' : 'CauseAndEffectExample::Effected2'[part_def])
      (connection_usage : 'CauseAndEffectExample::MultiCauseEffect'[connection_def])
      (connection_usage)
      (occurrence_usage 'a')
      (item_usage 'b')
      (part_usage 'c')
      (action_usage 'd')
      (not_implemented 'malformed')
      (not_implemented 'malformed')
      (reference_usage 'causeA' :> 'CauseAndEffectExample::a'[occurrence_usage])
      (reference_usage 'causeB' :> 'CauseAndEffectExample::b'[item_usage])
      (reference_usage 'effectC' :> 'CauseAndEffectExample::c'[part_usage])
      (reference_usage 'effectD' :> 'CauseAndEffectExample::d'[action_usage])
      (connection_usage)
      (connection_usage
        (connector_end 'a')
        (connector_end 'c'))
      (connection_usage
        (connector_end 'b')
        (connector_end 'd')
        (metadata_usage :> 'CausationMetadata'[unresolved]
          (feature_def 'isNecessary'
            (feature_value (=)))
          (feature_def 'probability'
            (feature_value (=))))))))
~~~
