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
  (document "derivation_connections.md"
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
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f8af119f4062e04e1a2458e205d7bb16ba42062275f2ea33965e0748f8ca657f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DerivationConnections"))) (kind "package") (name "DerivationConnections") (declared-name "DerivationConnections") (range (start (line 0) (character 0)) (end (line 0) (character 2305))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::Derivation"))) (kind "connection def") (name "Derivation") (declared-name "Derivation") (range (start (line 16) (character 1)) (end (line 16) (character 1660))) (parent (node (document "d0") (qualified-name "DerivationConnections"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::Derivation::"))) (kind "ref") (name "") (range (start (line 38) (character 2)) (end (line 38) (character 158))) (parent (node (document "d0") (qualified-name "DerivationConnections::Derivation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "derivedRequirements") (range (start (line 38) (character 22)) (end (line 38) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::Derivation::_documentation"))) (kind "documentation") (name "") (range (start (line 16) (character 1)) (end (line 16) (character 1660))) (parent (node (document "d0") (qualified-name "DerivationConnections::Derivation"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (kind "ref") (name "originalRequirement") (declared-name "originalRequirement") (range (start (line 35) (character 2)) (end (line 35) (character 132))) (parent (node (document "d0") (qualified-name "DerivationConnections::Derivation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "originalRequirements") (range (start (line 35) (character 45)) (end (line 35) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2305))) (parent (node (document "d0") (qualified-name "DerivationConnections"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::allTrue"))) (kind "import") (name "allTrue") (declared-name "allTrue") (range (start (line 7) (character 1)) (end (line 7) (character 42))) (parent (node (document "d0") (qualified-name "DerivationConnections"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::allTrue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 41))))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::derivations"))) (kind "connection def") (name "derivations") (declared-name "derivations") (range (start (line 59) (character 1)) (end (line 59) (character 131))) (parent (node (document "d0") (qualified-name "DerivationConnections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Derivation") (range (start (line 0) (character 0)) (end (line 0) (character 10)))))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::derivations::_documentation"))) (kind "documentation") (name "") (range (start (line 59) (character 1)) (end (line 59) (character 131))) (parent (node (document "d0") (qualified-name "DerivationConnections::derivations"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements"))) (kind "requirement") (name "derivedRequirements") (declared-name "derivedRequirements") (range (start (line 12) (character 1)) (end (line 12) (character 127))) (parent (node (document "d0") (qualified-name "DerivationConnections"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements::_documentation"))) (kind "documentation") (name "") (range (start (line 12) (character 1)) (end (line 12) (character 127))) (parent (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::excludes"))) (kind "import") (name "excludes") (declared-name "excludes") (range (start (line 6) (character 1)) (end (line 6) (character 44))) (parent (node (document "d0") (qualified-name "DerivationConnections"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::excludes") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 43))))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::originalRequirements"))) (kind "requirement") (name "originalRequirements") (declared-name "originalRequirements") (range (start (line 9) (character 1)) (end (line 9) (character 131))) (parent (node (document "d0") (qualified-name "DerivationConnections"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::originalRequirements::_documentation"))) (kind "documentation") (name "") (range (start (line 9) (character 1)) (end (line 9) (character 131))) (parent (node (document "d0") (qualified-name "DerivationConnections::originalRequirements"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DerivationConnections::Derivation::"))) (kind redefinition) (ordinal 0)) (authored-target "derivedRequirements") (range (start (line 38) (character 22)) (end (line 38) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements")))))
    (reference (id (source (node (document "d0") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (kind redefinition) (ordinal 0)) (authored-target "originalRequirements") (range (start (line 35) (character 45)) (end (line 35) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "DerivationConnections::originalRequirements")))))
    (reference (id (source (node (document "d0") (qualified-name "DerivationConnections::allTrue"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::allTrue") (range (start (line 7) (character 16)) (end (line 7) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DerivationConnections::derivations"))) (kind specialization) (ordinal 0)) (authored-target "Derivation") (range (start (line 0) (character 0)) (end (line 0) (character 10))) (outcome (status resolved) (target (node (document "d0") (qualified-name "DerivationConnections::Derivation")))))
    (reference (id (source (node (document "d0") (qualified-name "DerivationConnections::excludes"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::excludes") (range (start (line 6) (character 16)) (end (line 6) (character 43))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "DerivationConnections::Derivation::"))) (target (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DerivationConnections::Derivation::"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (target (node (document "d0") (qualified-name "DerivationConnections::originalRequirements"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "DerivationConnections::derivations"))) (target (node (document "d0") (qualified-name "DerivationConnections::Derivation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DerivationConnections::derivations"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
