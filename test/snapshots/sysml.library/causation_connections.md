# META
~~~ini
description=Standard Library: Domain Libraries/Cause and Effect/CausationConnections
type=file
~~~
# SOURCE
~~~sysml
standard library package CausationConnections {	
	doc 
	/* 
	 * This package provides a library model modeling causes, effects, and causation connections 
	 * between them.
	 */

	private import SequenceFunctions::isEmpty;
	private import SequenceFunctions::size;
	private import SequenceFunctions::intersection;
		 
	abstract occurrence causes[*] {
		doc /* Occurrences that are causes. */
	}
	
	abstract occurrence effects[*]  {
		doc /* Occurrences that are effects. */
	}
	
	abstract connection def Multicausation {
		doc
		/*
		 * A Multicausation connection models the situation in which one set of
		 * occurrences causes another.
		 * 
		 * To create a Multicausation connection, specialize this connection definition
		 * adding specific end features of the relavent types. Ends representing causes
		 * should subset 'causes', while ends representing effects should subset 'effects'.
		 * There must be at least one cause and at least one effect.
		 */
		 
		abstract constant ref occurrence causes[1..*] :>> causes :> participant {
			doc 
			/* 
			 * The causing occurrences. (Constant for each Multicausation instance.)
			 */
		}
		abstract constant ref occurrence effects[1..*] :>> effects :> participant {
			doc 
			/* 
			 * The effect occurrences caused by the causing occurrences. 
			 * (Constant for each Multicausation instance.)
			 */
		}
		
		private assert constraint disjointCauseEffect {
			doc /* causes must be disjoint from effects. */
			isEmpty(intersection(causes, effects))
		}
		
		private succession causalOrdering first [nCauses] causes.startShot then [nEffects] effects {
			doc /* All causes must exist before all effects. */
			attribute nCauses = size(causes);
			attribute nEffects = size(effects);
		}
	}
	
	abstract connection multicausations : Multicausation[*] {
		doc /* multicausations is the base feature for Multicausation ConnectionUsages. */
	}
	
	connection def Causation :> Multicausation {
		doc
		/*
		 * A Causation is a binary Multicausation in which a single cause occurrence
		 * causes a single effect occurrence. (However, a single cause can separately
		 * have multiple effects, and a single effect can have separate Causation
		 * connections with multiple causes.)
		 */
		
		end theCauses [*] occurrence theCause :> causes :>> source {
		    doc /* The single causing occurrence. */
		}
		
		end theEffects [*] occurrence theEffect :> effects :>> target {
			doc /* The single effect occurrence resulting from the cause. */
		}
	}
	
	abstract connection causations : Causation[*] :> multicausations {
		doc /* causations is the base feature for Causation ConnectionUsages. */
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/causation_connections.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 62) (end 31 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 64) (end 37 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_connection_definition_member")
        (source "semantic")
        (range (start 45 2) (end 48 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_connection_definition_member")
        (source "semantic")
        (range (start 50 2) (end 54 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:7d7a8611d8777e2a39efafe50706980f692c84b2ce1a330b4c687f2dbd7b97d3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::isEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::intersection") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Causation"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Multicausation"))))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Causation::theCauses"))) (kind connection) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Causation::theEffects"))) (kind connection) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::causes"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "participant")) (redefinition (reference "causes"))))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::effects"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "participant")) (redefinition (reference "effects"))))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::causations"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "multicausations"))))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::causes"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::effects"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::multicausations"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Multicausation"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::intersection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Causation"))) (kind specialization) (ordinal 0))
      (authored-target "Multicausation")
      (outcome (status resolved) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation")))))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::causes"))) (kind subsetting) (ordinal 0))
      (authored-target "participant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::causes"))) (kind redefinition) (ordinal 0))
      (authored-target "causes")
      (outcome (status resolved) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::causes")))))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::effects"))) (kind subsetting) (ordinal 0))
      (authored-target "participant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::effects"))) (kind redefinition) (ordinal 0))
      (authored-target "effects")
      (outcome (status resolved) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::effects")))))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::causations"))) (kind specialization) (ordinal 0))
      (authored-target "multicausations")
      (outcome (status resolved) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::multicausations")))))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::multicausations"))) (kind featureTyping) (ordinal 0))
      (authored-target "Multicausation")
      (outcome (status resolved) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Causation"))) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Causation"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::causes"))) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::causes"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::causes"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::effects"))) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::effects"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::effects"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::causations"))) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::multicausations"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::causations"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::multicausations"))) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::multicausations"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/causation_connections.md") (range (start 7 16) (end 7 42)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/causation_connections.md") (range (start 8 16) (end 8 39)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/causation_connections.md") (range (start 9 16) (end 9 47)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::intersection")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/causation_connections.md") (range (start 61 29) (end 61 43)) (probe (position 61 29))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Causation"))) (kind specialization) (ordinal 0) (authored-target "Multicausation")
      (outcome (status resolved) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation")))))
  )
  (query (document "memory://snapshot/causation_connections.md") (range (start 31 62) (end 31 73)) (probe (position 31 62))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::causes"))) (kind subsetting) (ordinal 0) (authored-target "participant")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/causation_connections.md") (range (start 31 52) (end 31 58)) (probe (position 31 52))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::causes"))) (kind redefinition) (ordinal 0) (authored-target "causes")
      (outcome (status resolved) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::causes")))))
  )
  (query (document "memory://snapshot/causation_connections.md") (range (start 37 64) (end 37 75)) (probe (position 37 64))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::effects"))) (kind subsetting) (ordinal 0) (authored-target "participant")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/causation_connections.md") (range (start 37 53) (end 37 60)) (probe (position 37 53))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::effects"))) (kind redefinition) (ordinal 0) (authored-target "effects")
      (outcome (status resolved) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation::effects")))))
  )
  (query (document "memory://snapshot/causation_connections.md") (range (start 79 50) (end 79 65)) (probe (position 79 50))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::causations"))) (kind specialization) (ordinal 0) (authored-target "multicausations")
      (outcome (status resolved) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::multicausations")))))
  )
  (query (document "memory://snapshot/causation_connections.md") (range (start 57 39) (end 57 53)) (probe (position 57 39))
    (reference (id (source (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::multicausations"))) (kind featureTyping) (ordinal 0) (authored-target "Multicausation")
      (outcome (status resolved) (target (node (document "memory://snapshot/causation_connections.md") (qualified-name "CausationConnections::Multicausation")))))
  )
)
~~~
