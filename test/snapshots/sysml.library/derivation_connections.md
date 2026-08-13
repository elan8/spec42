# META
~~~ini
description=Standard Library: Domain Libraries/Requirement Derivation/DerivationConnections
type=file
~~~
# SOURCE
~~~sysml
standard library package DerivationConnections {
	doc
	/*
	 * This package provides a library model for derivation connections between requirements.
	 */
	 
	private import SequenceFunctions::excludes;
	private import ControlFunctions::allTrue;
	
	requirement originalRequirements[*] {
		doc /* originalRequirements are the original requirements in Derivation connections. */
	}
	requirement derivedRequirements[*] {
		doc /* derivedRequirements are the derived requirments in Derivation connections. */
	}
	
	abstract connection def Derivation {
		doc
		/*
		 * A Derivation connection asserts that one or more derivedRequirements are derived from
		 * a single originalRequirement. This means that any subject that satisfies the
		 * originalRequirement should, in itself or though other things related to it, satisfy
		 * each of the derivedRequirements.
		 * 
		 * A connection usage typed by Derivation must have requirement usages for all its ends.
		 * The single end for the originalRequirement should subset originalRequirement, while
		 * the rest of the ends should subset derivedRequirements.
		 */
		
		// Note: This redefinition causes a distinguishibility problem for binary connections, becuse
		// participant is already redefined for them to limit the multiplicity to 2.
		// ref requirement :>> participant {
		//	doc /* All the participants in a Derivation must be requirements. */
		// }
		
		ref requirement originalRequirement[1] :>> originalRequirements :> participant {
			doc /* The single original requirement. */
		}
		ref requirement :>> derivedRequirements[1..*] :> participant {
			doc /* The one or more requirements that are derived from the original requirement. */
		}
		
		private assert constraint originalNotDerived {
			doc /* The original requirement must not be a derived requirement. */
			
			derivedRequirements->excludes(originalRequirement)
		}
		
		private assert constraint originalImpliesDerived {
			doc 
			/* 
			 * Whenever the originalRequirement is satisfied, all of the derivedRequirements must also
			 * be satisfied.
			 */
			 
			originalRequirement.result implies allTrue(derivedRequirements.result)
		}	
	}
	
	abstract connection derivations : Derivation[*] {
		doc /* derivations is the base feature for Derivation connection usages. */
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/derivation_connections.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_connection_definition_member")
        (source "semantic")
        (range (start 35 2) (end 37 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_connection_definition_member")
        (source "semantic")
        (range (start 38 2) (end 40 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 45 3) (end 45 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 55 3) (end 55 73))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:0700fa018a384b209497b8eb9d4814bd1732cd26b24f5e034d2d3fe1d87b9fce") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::excludes") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::allTrue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalImpliesDerived"))) (kind constraint) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived"))) (kind constraint) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivations"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Derivation"))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivedRequirements"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::originalRequirements"))) (kind requirement) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::excludes")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::allTrue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivations"))) (kind featureTyping) (ordinal 0))
      (authored-target "Derivation")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivations"))) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivations"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/derivation_connections.md") (range (start 6 16) (end 6 43)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::excludes")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 7 16) (end 7 41)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::allTrue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 59 35) (end 59 45)) (probe (position 59 35))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivations"))) (kind featureTyping) (ordinal 0) (authored-target "Derivation")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation")))))
  )
)
~~~
