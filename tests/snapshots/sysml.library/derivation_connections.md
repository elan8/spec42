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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 6 16) (end 6 43))
      )
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 69) (end 35 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 38 51) (end 38 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 3) (end 55 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 38) (end 55 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 46) (end 55 72))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:0700fa018a384b209497b8eb9d4814bd1732cd26b24f5e034d2d3fe1d87b9fce") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package provides a library model for derivation connections between requirements.\n\t "))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::excludes") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::allTrue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation"))) (kind connection-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * A Derivation connection asserts that one or more derivedRequirements are derived from\n\t\t * a single originalRequirement. This means that any subject that satisfies the\n\t\t * originalRequirement should, in itself or though other things related to it, satisfy\n\t\t * each of the derivedRequirements.\n\t\t * \n\t\t * A connection usage typed by Derivation must have requirement usages for all its ends.\n\t\t * The single end for the originalRequirement should subset originalRequirement, while\n\t\t * the rest of the ends should subset derivedRequirements.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (named (kind connection-def) (name "Derivation")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper unbounded))) (documentation (doc (text " The one or more requirements that are derived from the original requirement. "))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "participant")) (redefinition (reference "derivedRequirements")))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalImpliesDerived"))) (kind assert-constraint) (membership (kind feature) (visibility private)) (documentation (doc (text " \n\t\t\t * Whenever the originalRequirement is satisfied, all of the derivedRequirements must also\n\t\t\t * be satisfied.\n\t\t\t "))) (authored (membership (kind feature) (visibility private)) (relationships (memberAccessOperand (reference "originalRequirement::result")) (memberAccessOperand (reference "derivedRequirements::result")) (invocationCallee (reference "allTrue")))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived"))) (kind assert-constraint) (membership (kind feature) (visibility private)) (documentation (doc (text " The original requirement must not be a derived requirement. "))) (authored (membership (kind feature) (visibility private)) (relationships (expressionOperand (reference "derivedRequirements")) (expressionOperand (reference "originalRequirement")))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (documentation (doc (text " The single original requirement. "))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "participant")) (redefinition (reference "originalRequirements")))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivations"))) (kind connection-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text " derivations is the base feature for Derivation connection usages. "))) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Derivation")))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivedRequirements"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (documentation (doc (text " derivedRequirements are the derived requirments in Derivation connections. "))))
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::originalRequirements"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (documentation (doc (text " originalRequirements are the original requirements in Derivation connections. "))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::excludes")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::allTrue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (named (kind connection-def) (name "Derivation")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "participant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (named (kind connection-def) (name "Derivation")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "derivedRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivedRequirements")))))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalImpliesDerived"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "originalRequirement::result")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalImpliesDerived"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "derivedRequirements::result")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalImpliesDerived"))) (kind invocationCallee) (ordinal 0))
      (authored-target "allTrue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived"))) (kind expressionOperand) (ordinal 0))
      (authored-target "derivedRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivedRequirements")))))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived"))) (kind expressionOperand) (ordinal 1))
      (authored-target "originalRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement")))))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (kind subsetting) (ordinal 0))
      (authored-target "participant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (kind redefinition) (ordinal 0))
      (authored-target "originalRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::originalRequirements")))))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivations"))) (kind featureTyping) (ordinal 0))
      (authored-target "Derivation")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (named (kind connection-def) (name "Derivation")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivedRequirements"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (named (kind connection-def) (name "Derivation")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived"))) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivedRequirements"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived"))) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::originalRequirements"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivations"))) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivations"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (named (kind connection-def) (name "Derivation")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalImpliesDerived"))) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived"))) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalImpliesDerived"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation")))
      (subtype (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivations")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (named (kind connection-def) (name "Derivation")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation")))
      (supertype (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivedRequirements")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalImpliesDerived")))
      (featured-by (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation")))
    )
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived")))
      (featured-by (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation")))
    )
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement")))
      (featured-by (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation")))
      (supertype (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::originalRequirements")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivations")))
      (type (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation")) (provenance authored))
      (effective-type (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation")) (source direct))
      (supertype (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivedRequirements")))
      (subtype (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (named (kind connection-def) (name "Derivation")) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::originalRequirements")))
      (subtype (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/derivation_connections.md") (range (start 6 16) (end 6 43)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::excludes")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 7 16) (end 7 41)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::allTrue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 38 51) (end 38 62)) (probe (position 38 51))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (named (kind connection-def) (name "Derivation")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "participant")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 38 22) (end 38 41)) (probe (position 38 22))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (path (named (kind library-package) (name "DerivationConnections")) (named (kind connection-def) (name "Derivation")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "derivedRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivedRequirements")))))
    )
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 55 3) (end 55 29)) (probe (position 55 3))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalImpliesDerived"))) (kind memberAccessOperand) (ordinal 0) (authored-target "originalRequirement::result")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 55 46) (end 55 72)) (probe (position 55 46))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalImpliesDerived"))) (kind memberAccessOperand) (ordinal 1) (authored-target "derivedRequirements::result")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 55 38) (end 55 45)) (probe (position 55 38))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalImpliesDerived"))) (kind invocationCallee) (ordinal 0) (authored-target "allTrue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 45 3) (end 45 22)) (probe (position 45 3))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived"))) (kind expressionOperand) (ordinal 0) (authored-target "derivedRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivedRequirements")))))
    )
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 45 33) (end 45 52)) (probe (position 45 33))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalNotDerived"))) (kind expressionOperand) (ordinal 1) (authored-target "originalRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement")))))
    )
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 35 69) (end 35 80)) (probe (position 35 69))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (kind subsetting) (ordinal 0) (authored-target "participant")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 35 45) (end 35 65)) (probe (position 35 45))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (kind redefinition) (ordinal 0) (authored-target "originalRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::originalRequirements")))))
    )
  )
  (query (document "memory://snapshot/derivation_connections.md") (range (start 59 35) (end 59 45)) (probe (position 59 35))
    (reference (id (source (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::derivations"))) (kind featureTyping) (ordinal 0) (authored-target "Derivation")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_connections.md") (qualified-name "DerivationConnections::Derivation")))))
    )
  )
)
~~~
