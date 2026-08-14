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
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 40))
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
        (range (start 30 2) (end 34 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:e00e38e159b585f6e741150124d3440eddb44bad743def4b6355a75f1b0e1fd5") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RequirementDerivation") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_1"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_2"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Req1"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Req1_1"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Req1_2"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem1"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem2"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::satisfactionContext::system"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "System"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Subsystem1"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Subsystem2"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
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
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub1"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub2"))) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 1 16) (end 1 40)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (path (named (kind package) (name "RequirementDerivationExample")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "RequirementDerivation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 9 21) (end 9 25)) (probe (position 9 21))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (kind featureTyping) (ordinal 0) (authored-target "Req1")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1")))))
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 10 21) (end 10 27)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (kind featureTyping) (ordinal 0) (authored-target "Req1_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_1")))))
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 11 21) (end 11 27)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (kind featureTyping) (ordinal 0) (authored-target "Req1_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Req1_2")))))
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 18 15) (end 18 21)) (probe (position 18 15))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system"))) (kind featureTyping) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::System")))))
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 19 14) (end 19 24)) (probe (position 19 14))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub1"))) (kind featureTyping) (ordinal 0) (authored-target "Subsystem1")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem1")))))
  )
  (query (document "memory://snapshot/requirement_derivation_example.md") (range (start 20 14) (end 20 24)) (probe (position 20 14))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::system::sub2"))) (kind featureTyping) (ordinal 0) (authored-target "Subsystem2")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_derivation_example.md") (qualified-name "RequirementDerivationExample::Subsystem2")))))
  )
)
~~~
