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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "262edf7906a90100a59aa81b04ad6bf927fe5810a75ab0f6be1c856761694685") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind "package") (name "CauseAndEffectExample") (declared-name "CauseAndEffectExample"))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "CauseAndEffect::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1"))) (kind "part def") (name "Causer1") (declared-name "Causer1") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2"))) (kind "part def") (name "Causer2") (declared-name "Causer2") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1"))) (kind "part def") (name "Effected1") (declared-name "Effected1") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2"))) (kind "part def") (name "Effected2") (declared-name "Effected2") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect"))) (kind "connection def") (name "MultiCauseEffect") (declared-name "MultiCauseEffect") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (kind "interface end") (name "cause1") (declared-name "cause1") (parent (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect"))) (authored (relationships (typing (reference "Causer1")))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (kind "interface end") (name "cause2") (declared-name "cause2") (parent (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect"))) (authored (relationships (typing (reference "Causer2")))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (kind "interface end") (name "effect1") (declared-name "effect1") (parent (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect"))) (authored (relationships (typing (reference "Effected1")))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (kind "interface end") (name "effect2") (declared-name "effect2") (parent (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect"))) (authored (relationships (typing (reference "Effected2")))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_causation"))) (kind "metadata keyword") (name "causation") (declared-name "causation") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_causation#metadata_keyword"))) (kind "metadata keyword") (name "causation") (declared-name "causation") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_cause"))) (kind "metadata keyword") (name "cause") (declared-name "cause") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_cause#metadata_keyword"))) (kind "metadata keyword") (name "cause") (declared-name "cause") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef"))) (kind "connection def") (name "_connectionDef") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause"))) (kind "interface end") (name "#cause") (declared-name "#cause") (parent (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef"))) (authored (relationships (reference-subsetting (reference "a")))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause#interface_end"))) (kind "interface end") (name "#cause") (declared-name "#cause") (parent (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef"))) (authored (relationships (reference-subsetting (reference "b")))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect"))) (kind "interface end") (name "#effect") (declared-name "#effect") (parent (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef"))) (authored (relationships (reference-subsetting (reference "c")))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect#interface_end"))) (kind "interface end") (name "#effect") (declared-name "#effect") (parent (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef"))) (authored (relationships (reference-subsetting (reference "d")))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_effect"))) (kind "metadata keyword") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_effect#metadata_keyword"))) (kind "metadata keyword") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_multicausation"))) (kind "metadata keyword") (name "multicausation") (declared-name "multicausation") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_multicausation#metadata_keyword"))) (kind "metadata keyword") (name "multicausation") (declared-name "multicausation") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::_multicausation#metadata_keyword2"))) (kind "metadata keyword") (name "multicausation") (declared-name "multicausation") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::a"))) (kind "occurrence") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::b"))) (kind "item def") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::c"))) (kind "part") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::causer1"))) (kind "part") (name "causer1") (declared-name "causer1") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Causer1")))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::causer2"))) (kind "part") (name "causer2") (declared-name "causer2") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Causer2")))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::connection"))) (kind "kermlDecl") (name "connection") (declared-name "connection") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::d"))) (kind "action") (name "d") (declared-name "d") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::effected1"))) (kind "part") (name "effected1") (declared-name "effected1") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Effected1")))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffectExample::effected2"))) (kind "part") (name "effected2") (declared-name "effected2") (parent (node (document "d0") (qualified-name "CauseAndEffectExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Effected2")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind connectionSource) (ordinal 0)) (authored-target "a") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::a")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind connectionSource) (ordinal 1)) (authored-target "b") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::b")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind connectionTarget) (ordinal 0)) (authored-target "c") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::c")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind connectionTarget) (ordinal 1)) (authored-target "d") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::d")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "CauseAndEffect::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (kind featureTyping) (ordinal 0)) (authored-target "Causer1") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (kind featureTyping) (ordinal 0)) (authored-target "Causer2") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (kind featureTyping) (ordinal 0)) (authored-target "Effected1") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (kind featureTyping) (ordinal 0)) (authored-target "Effected2") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "a") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::a")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause#interface_end"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "b") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::b")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "c") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::c")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect#interface_end"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "d") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::d")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::causer1"))) (kind featureTyping) (ordinal 0)) (authored-target "Causer1") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::causer2"))) (kind featureTyping) (ordinal 0)) (authored-target "Causer2") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::effected1"))) (kind featureTyping) (ordinal 0)) (authored-target "Effected1") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffectExample::effected2"))) (kind featureTyping) (ordinal 0)) (authored-target "Effected2") (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2")))))
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
    (relationship (kind connection) (source (node (document "d0") (qualified-name "CauseAndEffectExample::a"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::c"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "a") (target "c")))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "CauseAndEffectExample::b"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::d"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample"))) (kind connectionSource) (ordinal 1)) (expression (kind connection) (source "b") (target "d")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CauseAndEffectExample::causer1"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::causer1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CauseAndEffectExample::causer2"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Causer2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::causer2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CauseAndEffectExample::effected1"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::effected1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CauseAndEffectExample::effected2"))) (target (node (document "d0") (qualified-name "CauseAndEffectExample::Effected2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffectExample::effected2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 34 17) (end 34 18)) (probe (position 34 17))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "a")
        (range (start 34 17) (end 34 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffectExample::a") (range (start 28 15) (end 28 17)))
        )
      )
    )
    (query (range (start 35 17) (end 35 18)) (probe (position 35 17))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#cause#interface_end"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "b")
        (range (start 35 17) (end 35 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffectExample::b") (range (start 29 4) (end 29 11)))
        )
      )
    )
    (query (range (start 36 18) (end 36 19)) (probe (position 36 18))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "c")
        (range (start 36 18) (end 36 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffectExample::c") (range (start 30 4) (end 30 11)))
        )
      )
    )
    (query (range (start 37 18) (end 37 19)) (probe (position 37 18))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample::_connectionDef::#effect#interface_end"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "d")
        (range (start 37 18) (end 37 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffectExample::d") (range (start 31 4) (end 31 13)))
        )
      )
    )
    (query (range (start 47 20) (end 47 21)) (probe (position 47 20))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample"))
        (kind connectionSource) (ordinal 0) (authored-target "a")
        (range (start 47 20) (end 47 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffectExample::a") (range (start 28 15) (end 28 17)))
        )
      )
    )
    (query (range (start 47 25) (end 47 26)) (probe (position 47 25))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample"))
        (kind connectionTarget) (ordinal 0) (authored-target "c")
        (range (start 47 25) (end 47 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffectExample::c") (range (start 30 4) (end 30 11)))
        )
      )
    )
    (query (range (start 48 20) (end 48 21)) (probe (position 48 20))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample"))
        (kind connectionSource) (ordinal 1) (authored-target "b")
        (range (start 48 20) (end 48 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffectExample::b") (range (start 29 4) (end 29 11)))
        )
      )
    )
    (query (range (start 48 25) (end 48 26)) (probe (position 48 25))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample"))
        (kind connectionTarget) (ordinal 1) (authored-target "d")
        (range (start 48 25) (end 48 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffectExample::d") (range (start 31 4) (end 31 13)))
        )
      )
    )
    (query (range (start 15 16) (end 15 23)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample::causer1"))
        (kind featureTyping) (ordinal 0) (authored-target "Causer1")
        (range (start 15 16) (end 15 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffectExample::Causer1") (range (start 3 1) (end 3 18)))
        )
      )
    )
    (query (range (start 16 16) (end 16 23)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample::causer2"))
        (kind featureTyping) (ordinal 0) (authored-target "Causer2")
        (range (start 16 16) (end 16 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffectExample::Causer2") (range (start 4 1) (end 4 18)))
        )
      )
    )
    (query (range (start 17 18) (end 17 27)) (probe (position 17 18))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample::effected1"))
        (kind featureTyping) (ordinal 0) (authored-target "Effected1")
        (range (start 17 18) (end 17 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffectExample::Effected1") (range (start 5 1) (end 5 20)))
        )
      )
    )
    (query (range (start 18 18) (end 18 27)) (probe (position 18 18))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample::effected2"))
        (kind featureTyping) (ordinal 0) (authored-target "Effected2")
        (range (start 18 18) (end 18 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffectExample::Effected2") (range (start 6 1) (end 6 20)))
        )
      )
    )
    (query (range (start 1 16) (end 1 30)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffectExample::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "CauseAndEffect::*")
        (range (start 1 16) (end 1 30))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
