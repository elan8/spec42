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
  (document "causation_connections.md"
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
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "64f638b154aae1bcd2b94a69981c968524dc431e2f075196deea1ec7b5b94dbb") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CausationConnections"))) (kind "package") (name "CausationConnections") (declared-name "CausationConnections"))
    (element (id (node (document "d0") (qualified-name "CausationConnections::Causation"))) (kind "connection def") (name "Causation") (declared-name "Causation") (parent (node (document "d0") (qualified-name "CausationConnections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Multicausation")))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::Causation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "CausationConnections::Causation"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::Causation::theCauses"))) (kind "interface end") (name "theCauses") (declared-name "theCauses") (parent (node (document "d0") (qualified-name "CausationConnections::Causation"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::Causation::theEffects"))) (kind "interface end") (name "theEffects") (declared-name "theEffects") (parent (node (document "d0") (qualified-name "CausationConnections::Causation"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::Multicausation"))) (kind "connection def") (name "Multicausation") (declared-name "Multicausation") (parent (node (document "d0") (qualified-name "CausationConnections"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::Multicausation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "CausationConnections::Multicausation"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "CausationConnections"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::causations"))) (kind "connection def") (name "causations") (declared-name "causations") (parent (node (document "d0") (qualified-name "CausationConnections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "multicausations")))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::causations::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "CausationConnections::causations"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::causes"))) (kind "occurrence") (name "causes") (declared-name "causes") (parent (node (document "d0") (qualified-name "CausationConnections"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::causes::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "CausationConnections::causes"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::effects"))) (kind "occurrence") (name "effects") (declared-name "effects") (parent (node (document "d0") (qualified-name "CausationConnections"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::effects::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "CausationConnections::effects"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::intersection"))) (kind "import") (name "intersection") (declared-name "intersection") (parent (node (document "d0") (qualified-name "CausationConnections"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::intersection") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (parent (node (document "d0") (qualified-name "CausationConnections"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::multicausations"))) (kind "connection def") (name "multicausations") (declared-name "multicausations") (parent (node (document "d0") (qualified-name "CausationConnections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Multicausation")))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::multicausations::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "CausationConnections::multicausations"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::size"))) (kind "import") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "CausationConnections"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::Causation"))) (kind specialization) (ordinal 0)) (authored-target "Multicausation") (outcome (status resolved) (target (node (document "d0") (qualified-name "CausationConnections::Multicausation")))))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::Causation::theCauses"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "CausationConnections::Causation::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::Causation::theEffects"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "CausationConnections::Causation::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::causations"))) (kind specialization) (ordinal 0)) (authored-target "multicausations") (outcome (status resolved) (target (node (document "d0") (qualified-name "CausationConnections::multicausations")))))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::intersection"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::intersection") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::multicausations"))) (kind specialization) (ordinal 0)) (authored-target "Multicausation") (outcome (status resolved) (target (node (document "d0") (qualified-name "CausationConnections::Multicausation")))))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CausationConnections::Causation"))) (target (node (document "d0") (qualified-name "CausationConnections::Multicausation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CausationConnections::Causation"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CausationConnections::Causation::theCauses"))) (target (node (document "d0") (qualified-name "CausationConnections::Causation::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CausationConnections::Causation::theCauses"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CausationConnections::Causation::theEffects"))) (target (node (document "d0") (qualified-name "CausationConnections::Causation::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CausationConnections::Causation::theEffects"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CausationConnections::causations"))) (target (node (document "d0") (qualified-name "CausationConnections::multicausations"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CausationConnections::causations"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CausationConnections::multicausations"))) (target (node (document "d0") (qualified-name "CausationConnections::Multicausation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CausationConnections::multicausations"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 0 0) (end 0 14)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "CausationConnections::multicausations"))
        (kind specialization) (ordinal 0) (authored-target "Multicausation")
        (range (start 0 0) (end 0 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CausationConnections::Multicausation") (range (start 19 1) (end 19 1272)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "CausationConnections::causations"))
        (kind specialization) (ordinal 0) (authored-target "multicausations")
        (range (start 0 0) (end 0 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CausationConnections::multicausations") (range (start 57 1) (end 57 146)))
        )
      )
    )
    (query (range (start 61 29) (end 61 43)) (probe (position 61 29))
      (reference
        (source (document "d0") (qualified-name "CausationConnections::Causation"))
        (kind specialization) (ordinal 0) (authored-target "Multicausation")
        (range (start 61 29) (end 61 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CausationConnections::Multicausation") (range (start 19 1) (end 19 1272)))
        )
      )
    )
    (query (range (start 0 0) (end 0 15)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "CausationConnections::multicausations"))
        (kind specialization) (ordinal 0) (authored-target "Multicausation")
        (range (start 0 0) (end 0 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CausationConnections::Multicausation") (range (start 19 1) (end 19 1272)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "CausationConnections::causations"))
        (kind specialization) (ordinal 0) (authored-target "multicausations")
        (range (start 0 0) (end 0 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CausationConnections::multicausations") (range (start 57 1) (end 57 146)))
        )
      )
    )
    (query (range (start 8 16) (end 8 39)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "CausationConnections::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 8 16) (end 8 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 42)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "CausationConnections::isEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
        (range (start 7 16) (end 7 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 47)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "CausationConnections::intersection"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::intersection")
        (range (start 9 16) (end 9 47))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
