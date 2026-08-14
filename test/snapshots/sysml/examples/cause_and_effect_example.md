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
  (document "memory://snapshot/cause_and_effect_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 1) (end 8 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 20 1) (end 20 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 20 17) (end 22 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 20 17) (end 22 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 24 1) (end 24 17))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 24 17) (end 28 4))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 24 17) (end 28 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 33 1) (end 33 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 40 1) (end 40 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 40 8) (end 41 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 41 1) (end 41 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 41 8) (end 42 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 42 1) (end 42 9))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 42 9) (end 43 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 43 1) (end 43 9))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 43 9) (end 45 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 45 1) (end 45 17))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 45 17) (end 47 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 47 1) (end 47 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 48 1) (end 48 12))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:69f74b0751c962ce0cd0ebd8f57bdd0faafd0b4bd08338cdef5e11d903a54859") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "CauseAndEffect") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind connection-def) (ordinal 0))))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "a")) (connectorEnd (reference "c")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 1))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "b")) (connectorEnd (reference "d")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Causer1")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Causer2")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Effected1")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Effected2")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::a"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::b"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::c"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Causer1")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Causer2")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::d"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Effected1")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Effected2")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "CauseAndEffect")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::a")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::b")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::c")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 1))
      (authored-target "d")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::d")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Causer1")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Causer2")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Effected1")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Effected2")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Causer1")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Causer2")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Effected1")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Effected2")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2")))))
  )
  (relationships
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::d"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer1"))) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer2"))) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected1"))) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected2"))) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1")))
      (subtype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1")) (scopes any))
      (subtype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2")))
      (subtype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2")) (scopes any))
      (subtype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1")))
      (subtype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1")) (scopes any))
      (subtype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2")))
      (subtype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2")) (scopes any))
      (subtype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1")))
      (featured-by (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect")))
      (type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1")) (source direct))
      (supertype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2")))
      (featured-by (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect")))
      (type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2")) (provenance authored))
      (effective-type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2")) (source direct))
      (supertype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1")))
      (featured-by (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect")))
      (type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1")) (source direct))
      (supertype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2")))
      (featured-by (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect")))
      (type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2")) (provenance authored))
      (effective-type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2")) (source direct))
      (supertype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer1")))
      (type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1")) (source direct))
      (supertype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer2")))
      (type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2")) (provenance authored))
      (effective-type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2")) (source direct))
      (supertype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected1")))
      (type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1")) (source direct))
      (supertype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected2")))
      (type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2")) (provenance authored))
      (effective-type (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2")) (source direct))
      (supertype (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 1 16) (end 1 33)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "CauseAndEffect")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 47 20) (end 47 21)) (probe (position 47 20))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::a")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 48 20) (end 48 21)) (probe (position 48 20))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::b")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 47 25) (end 47 26)) (probe (position 47 25))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::c")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 48 25) (end 48 26)) (probe (position 48 25))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (path (named (kind package) (name "CauseAndEffectExample")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 1) (authored-target "d")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::d")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 9 22) (end 9 29)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause1"))) (kind featureTyping) (ordinal 0) (authored-target "Causer1")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 10 22) (end 10 29)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::cause2"))) (kind featureTyping) (ordinal 0) (authored-target "Causer2")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 11 24) (end 11 33)) (probe (position 11 24))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect1"))) (kind featureTyping) (ordinal 0) (authored-target "Effected1")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 12 24) (end 12 33)) (probe (position 12 24))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::MultiCauseEffect::effect2"))) (kind featureTyping) (ordinal 0) (authored-target "Effected2")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 15 16) (end 15 23)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer1"))) (kind featureTyping) (ordinal 0) (authored-target "Causer1")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer1")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 16 16) (end 16 23)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::causer2"))) (kind featureTyping) (ordinal 0) (authored-target "Causer2")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Causer2")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 17 18) (end 17 27)) (probe (position 17 18))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected1"))) (kind featureTyping) (ordinal 0) (authored-target "Effected1")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected1")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect_example.md") (range (start 18 18) (end 18 27)) (probe (position 18 18))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::effected2"))) (kind featureTyping) (ordinal 0) (authored-target "Effected2")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect_example.md") (qualified-name "CauseAndEffectExample::Effected2")))))
    )
  )
)
~~~
