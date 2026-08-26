# META
~~~ini
description=SysML Example (Requirements): RequirementDerivationExample
type=file
~~~
# SOURCE
~~~sysml
package RequirementDerivationExample {
	private import RequirementDerivation::*;
	
	requirement def Req1;
	
	requirement def Req1_1;
	requirement def Req1_2;
	
	#derivation connection def Req1_Derivation {
		end #original r1 : Req1;
		end #derive r1_1 : Req1_1;
		end #derive r1_2 : Req1_2;
	}
	
	part def System;
	part def Subsystem1;
	part def Subsystem2;
	
	part system : System {
		part sub1 : Subsystem1;
		part sub2 : Subsystem2;
	}
	
	part satisfactionContext {
		ref :>> system;
		
		satisfy requirement req1 : Req1 by system;
		satisfy requirement req1_1 : Req1_1 by system.sub1;
		satisfy requirement req1_2 : Req1_2 by system.sub2;
		
		#derivation connection : Req1_Derivation {
			end r1 ::> req1;
			end r1_1 ::> req1_1;
			end r1_2 ::> req1_1;
		}
		
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/requirement_derivation_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 40))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 23 1) (end 36 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 26 2) (end 26 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 27 2) (end 27 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 28 2) (end 28 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 30 2) (end 30 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 14) (end 31 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 16) (end 32 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 33 16) (end 33 22))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:e00e38e159b585f6e741150124d3440eddb44bad743def4b6355a75f1b0e1fd5") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RequirementDerivation") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_1"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_2"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Req1")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Req1_1")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 2)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Req1_2")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem1"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem2"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "system")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0))))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Req1_Derivation")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1"))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "req1")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1_1"))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "req1_1")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1_2"))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 2)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "req1_1")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "System")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Subsystem1")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Subsystem2")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RequirementDerivation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Req1")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1")))))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Req1_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_1")))))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Req1_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_2")))))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Req1_Derivation")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation")))))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system")))))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1"))))) (kind connectorEnd) (ordinal 0))
      (authored-target "req1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1_1"))))) (kind connectorEnd) (ordinal 0))
      (authored-target "req1_1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1_2"))))) (kind connectorEnd) (ordinal 0))
      (authored-target "req1_1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))) (kind featureTyping) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System")))))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Subsystem1")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem1")))))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Subsystem2")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem2")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub1"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub2"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1"))))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1_1"))))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1_2"))))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub1"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub2"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1")))
      (subtype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_1")))
      (subtype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_2")))
      (subtype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation")))
      (positional-ends (authored 3) (effective 3))
      (subtype (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1")))
      (featured-by (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation")))
      (type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1")))
      (featured-by (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation")))
      (type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_1")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2")))
      (featured-by (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation")))
      (type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_2")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_2")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem1")))
      (subtype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem2")))
      (subtype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System")))
      (subtype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::satisfactionContext")))
      (effective-type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System")) (source inherited) (from (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))))
      (supertype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System")) (scopes any))
      (supertype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)))))
      (positional-ends (authored 3) (effective 3))
      (featured-by (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::satisfactionContext")))
      (type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1")))))
      (featured-by (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1_1")))))
      (featured-by (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1_2")))))
      (featured-by (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system")))
      (type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System")) (scopes any))
      (subtype (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub1")))
      (featured-by (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system")))
      (type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem1")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub2")))
      (featured-by (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system")))
      (type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem2")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem2")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem2")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 1 16) (end 1 40)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "RequirementDerivation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 9 21) (end 9 25)) (probe (position 9 21))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (kind featureTyping) (ordinal 0) (authored-target "Req1")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1")))))
    )
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 10 21) (end 10 27)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (kind featureTyping) (ordinal 0) (authored-target "Req1_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_1")))))
    )
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 11 21) (end 11 27)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (kind featureTyping) (ordinal 0) (authored-target "Req1_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_2")))))
    )
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 30 27) (end 30 42)) (probe (position 30 27))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Req1_Derivation")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation")))))
    )
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 24 10) (end 24 16)) (probe (position 24 10))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system")))))
    )
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 31 14) (end 31 18)) (probe (position 31 14))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1"))))) (kind connectorEnd) (ordinal 0) (authored-target "req1")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 32 16) (end 32 22)) (probe (position 32 16))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1_1"))))) (kind connectorEnd) (ordinal 0) (authored-target "req1_1")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 33 16) (end 33 22)) (probe (position 33 16))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (named (kind part) (name "satisfactionContext")) (anonymous (kind connection) (ordinal 0)) (named (kind connection) (name "r1_2"))))) (kind connectorEnd) (ordinal 0) (authored-target "req1_1")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 18 15) (end 18 21)) (probe (position 18 15))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))) (kind featureTyping) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System")))))
    )
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 19 14) (end 19 24)) (probe (position 19 14))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub1"))) (kind featureTyping) (ordinal 0) (authored-target "Subsystem1")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem1")))))
    )
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 20 14) (end 20 24)) (probe (position 20 14))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub2"))) (kind featureTyping) (ordinal 0) (authored-target "Subsystem2")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem2")))))
    )
  )
)
~~~
