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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "cause_and_effect_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 30))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 24 17) (end 24 126))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 30 4) (end 30 11))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 40 8) (end 40 23))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 41 8) (end 41 23))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 42 9) (end 42 25))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 43 9) (end 43 27))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 45 17) (end 45 66))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 47 20) (end 47 21))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 48 20) (end 48 21))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "262edf7906a90100a59aa81b04ad6bf927fe5810a75ab0f6be1c856761694685") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind "package") (name "CauseAndEffectExample") (declared-name "CauseAndEffectExample") (range (start (line 0) (character 0)) (end (line 0) (character 1197))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 34))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "CauseAndEffect::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 30))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1"))) (kind "part def") (name "Causer1") (declared-name "Causer1") (range (start (line 3) (character 1)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2"))) (kind "part def") (name "Causer2") (declared-name "Causer2") (range (start (line 4) (character 1)) (end (line 4) (character 18))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1"))) (kind "part def") (name "Effected1") (declared-name "Effected1") (range (start (line 5) (character 1)) (end (line 5) (character 20))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2"))) (kind "part def") (name "Effected2") (declared-name "Effected2") (range (start (line 6) (character 1)) (end (line 6) (character 20))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect"))) (kind "connection def") (name "MultiCauseEffect") (declared-name "MultiCauseEffect") (range (start (line 8) (character 1)) (end (line 8) (character 185))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (kind "interface end") (name "cause1") (declared-name "cause1") (range (start (line 9) (character 2)) (end (line 9) (character 30))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect"))) (authored (relationships (typing (reference "Causer1") (range none)))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (kind "interface end") (name "cause2") (declared-name "cause2") (range (start (line 10) (character 2)) (end (line 10) (character 30))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect"))) (authored (relationships (typing (reference "Causer2") (range none)))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (kind "interface end") (name "effect1") (declared-name "effect1") (range (start (line 11) (character 2)) (end (line 11) (character 34))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect"))) (authored (relationships (typing (reference "Effected1") (range none)))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (kind "interface end") (name "effect2") (declared-name "effect2") (range (start (line 12) (character 2)) (end (line 12) (character 34))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect"))) (authored (relationships (typing (reference "Effected2") (range none)))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_causation"))) (kind "metadata keyword") (name "causation") (declared-name "causation") (range (start (line 47) (character 1)) (end (line 47) (character 12))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_causation#metadata_keyword"))) (kind "metadata keyword") (name "causation") (declared-name "causation") (range (start (line 48) (character 1)) (end (line 48) (character 12))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_cause"))) (kind "metadata keyword") (name "cause") (declared-name "cause") (range (start (line 40) (character 1)) (end (line 40) (character 8))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_cause#metadata_keyword"))) (kind "metadata keyword") (name "cause") (declared-name "cause") (range (start (line 41) (character 1)) (end (line 41) (character 8))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef"))) (kind "connection def") (name "_connectionDef") (range (start (line 33) (character 1)) (end (line 33) (character 114))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause"))) (kind "interface end") (name "#cause") (declared-name "#cause") (range (start (line 34) (character 2)) (end (line 34) (character 19))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef"))) (authored (relationships (reference-subsetting (reference "a") (range (start (line 34) (character 17)) (end (line 34) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause#interface_end"))) (kind "interface end") (name "#cause") (declared-name "#cause") (range (start (line 35) (character 2)) (end (line 35) (character 19))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef"))) (authored (relationships (reference-subsetting (reference "b") (range (start (line 35) (character 17)) (end (line 35) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect"))) (kind "interface end") (name "#effect") (declared-name "#effect") (range (start (line 36) (character 2)) (end (line 36) (character 20))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef"))) (authored (relationships (reference-subsetting (reference "c") (range (start (line 36) (character 18)) (end (line 36) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect#interface_end"))) (kind "interface end") (name "#effect") (declared-name "#effect") (range (start (line 37) (character 2)) (end (line 37) (character 20))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef"))) (authored (relationships (reference-subsetting (reference "d") (range (start (line 37) (character 18)) (end (line 37) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_effect"))) (kind "metadata keyword") (name "effect") (declared-name "effect") (range (start (line 42) (character 1)) (end (line 42) (character 9))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_effect#metadata_keyword"))) (kind "metadata keyword") (name "effect") (declared-name "effect") (range (start (line 43) (character 1)) (end (line 43) (character 9))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_multicausation"))) (kind "metadata keyword") (name "multicausation") (declared-name "multicausation") (range (start (line 20) (character 1)) (end (line 20) (character 17))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_multicausation#metadata_keyword"))) (kind "metadata keyword") (name "multicausation") (declared-name "multicausation") (range (start (line 24) (character 1)) (end (line 24) (character 17))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_multicausation#metadata_keyword2"))) (kind "metadata keyword") (name "multicausation") (declared-name "multicausation") (range (start (line 45) (character 1)) (end (line 45) (character 17))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::a"))) (kind "occurrence") (name "a") (declared-name "a") (range (start (line 28) (character 15)) (end (line 28) (character 17))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::b"))) (kind "item def") (name "b") (declared-name "b") (range (start (line 29) (character 4)) (end (line 29) (character 11))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::c"))) (kind "part") (name "c") (declared-name "c") (range (start (line 30) (character 4)) (end (line 30) (character 11))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::causer1"))) (kind "part") (name "causer1") (declared-name "causer1") (range (start (line 15) (character 1)) (end (line 15) (character 24))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Causer1") (range (start (line 15) (character 16)) (end (line 15) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::causer2"))) (kind "part") (name "causer2") (declared-name "causer2") (range (start (line 16) (character 1)) (end (line 16) (character 24))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Causer2") (range (start (line 16) (character 16)) (end (line 16) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::connection"))) (kind "kermlDecl") (name "connection") (declared-name "connection") (range (start (line 20) (character 17)) (end (line 20) (character 150))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::d"))) (kind "action") (name "d") (declared-name "d") (range (start (line 31) (character 4)) (end (line 31) (character 13))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::effected1"))) (kind "part") (name "effected1") (declared-name "effected1") (range (start (line 17) (character 1)) (end (line 17) (character 28))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Effected1") (range (start (line 17) (character 18)) (end (line 17) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::effected2"))) (kind "part") (name "effected2") (declared-name "effected2") (range (start (line 18) (character 1)) (end (line 18) (character 28))) (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Effected2") (range (start (line 18) (character 18)) (end (line 18) (character 27)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind connectionSource) (ordinal 0)) (authored-target "a") (range (start (line 47) (character 20)) (end (line 47) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::a")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind connectionSource) (ordinal 1)) (authored-target "b") (range (start (line 48) (character 20)) (end (line 48) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::b")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind connectionTarget) (ordinal 0)) (authored-target "c") (range (start (line 47) (character 25)) (end (line 47) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::c")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind connectionTarget) (ordinal 1)) (authored-target "d") (range (start (line 48) (character 25)) (end (line 48) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::d")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "CauseAndEffect::*") (range (start (line 1) (character 16)) (end (line 1) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (kind featureTyping) (ordinal 0)) (authored-target "Causer1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (kind featureTyping) (ordinal 0)) (authored-target "Causer2") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (kind featureTyping) (ordinal 0)) (authored-target "Effected1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (kind featureTyping) (ordinal 0)) (authored-target "Effected2") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "a") (range (start (line 34) (character 17)) (end (line 34) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::a")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause#interface_end"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "b") (range (start (line 35) (character 17)) (end (line 35) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::b")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "c") (range (start (line 36) (character 18)) (end (line 36) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::c")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect#interface_end"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "d") (range (start (line 37) (character 18)) (end (line 37) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::d")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::causer1"))) (kind featureTyping) (ordinal 0)) (authored-target "Causer1") (range (start (line 15) (character 16)) (end (line 15) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::causer2"))) (kind featureTyping) (ordinal 0)) (authored-target "Causer2") (range (start (line 16) (character 16)) (end (line 16) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::effected1"))) (kind featureTyping) (ordinal 0)) (authored-target "Effected1") (range (start (line 17) (character 18)) (end (line 17) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::effected2"))) (kind featureTyping) (ordinal 0)) (authored-target "Effected2") (range (start (line 18) (character 18)) (end (line 18) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::a"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause#interface_end"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::b"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause#interface_end"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::c"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect#interface_end"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::d"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect#interface_end"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "CauseAndEffectExample::a"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::c"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "a") (target "c") (source-range (start (line 47) (character 20)) (end (line 47) (character 21))) (target-range (start (line 47) (character 25)) (end (line 47) (character 26)))))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "CauseAndEffectExample::b"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::d"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind connectionSource) (ordinal 1)) (expression (kind connection) (source "b") (target "d") (source-range (start (line 48) (character 20)) (end (line 48) (character 21))) (target-range (start (line 48) (character 25)) (end (line 48) (character 26)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CauseAndEffectExample::causer1"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::causer1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CauseAndEffectExample::causer2"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::causer2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CauseAndEffectExample::effected1"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::effected1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CauseAndEffectExample::effected2"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::effected2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
