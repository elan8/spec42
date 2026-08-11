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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f8af119f4062e04e1a2458e205d7bb16ba42062275f2ea33965e0748f8ca657f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DerivationConnections"))) (kind "package") (name "DerivationConnections") (declared-name "DerivationConnections"))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::Derivation"))) (kind "connection def") (name "Derivation") (declared-name "Derivation") (parent (node (document "d0") (qualified-name "DerivationConnections"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::Derivation::"))) (kind "ref") (name "") (parent (node (document "d0") (qualified-name "DerivationConnections::Derivation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "derivedRequirements")))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::Derivation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "DerivationConnections::Derivation"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (kind "ref") (name "originalRequirement") (declared-name "originalRequirement") (parent (node (document "d0") (qualified-name "DerivationConnections::Derivation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "originalRequirements")))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "DerivationConnections"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::allTrue"))) (kind "import") (name "allTrue") (declared-name "allTrue") (parent (node (document "d0") (qualified-name "DerivationConnections"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::allTrue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::derivations"))) (kind "connection def") (name "derivations") (declared-name "derivations") (parent (node (document "d0") (qualified-name "DerivationConnections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Derivation")))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::derivations::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "DerivationConnections::derivations"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements"))) (kind "requirement") (name "derivedRequirements") (declared-name "derivedRequirements") (parent (node (document "d0") (qualified-name "DerivationConnections"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::excludes"))) (kind "import") (name "excludes") (declared-name "excludes") (parent (node (document "d0") (qualified-name "DerivationConnections"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::excludes") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::originalRequirements"))) (kind "requirement") (name "originalRequirements") (declared-name "originalRequirements") (parent (node (document "d0") (qualified-name "DerivationConnections"))))
    (element (id (node (document "d0") (qualified-name "DerivationConnections::originalRequirements::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "DerivationConnections::originalRequirements"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DerivationConnections::Derivation::"))) (kind redefinition) (ordinal 0)) (authored-target "derivedRequirements") (outcome (status resolved) (target (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements")))))
    (reference (id (source (node (document "d0") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (kind redefinition) (ordinal 0)) (authored-target "originalRequirements") (outcome (status resolved) (target (node (document "d0") (qualified-name "DerivationConnections::originalRequirements")))))
    (reference (id (source (node (document "d0") (qualified-name "DerivationConnections::allTrue"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::allTrue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DerivationConnections::derivations"))) (kind specialization) (ordinal 0)) (authored-target "Derivation") (outcome (status resolved) (target (node (document "d0") (qualified-name "DerivationConnections::Derivation")))))
    (reference (id (source (node (document "d0") (qualified-name "DerivationConnections::excludes"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::excludes") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 0 0) (end 0 10)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "DerivationConnections::derivations"))
        (kind specialization) (ordinal 0) (authored-target "Derivation")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "DerivationConnections::Derivation") (range (start 16 1) (end 16 1660)))
        )
      )
    )
    (query (range (start 38 22) (end 38 41)) (probe (position 38 22))
      (reference
        (source (document "d0") (qualified-name "DerivationConnections::Derivation::"))
        (kind redefinition) (ordinal 0) (authored-target "derivedRequirements")
        (range (start 38 22) (end 38 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "DerivationConnections::derivedRequirements") (range (start 12 1) (end 12 127)))
        )
      )
    )
    (query (range (start 35 45) (end 35 65)) (probe (position 35 45))
      (reference
        (source (document "d0") (qualified-name "DerivationConnections::Derivation::originalRequirement"))
        (kind redefinition) (ordinal 0) (authored-target "originalRequirements")
        (range (start 35 45) (end 35 65))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "DerivationConnections::originalRequirements") (range (start 9 1) (end 9 131)))
        )
      )
    )
    (query (range (start 7 16) (end 7 41)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "DerivationConnections::allTrue"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::allTrue")
        (range (start 7 16) (end 7 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 43)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "DerivationConnections::excludes"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::excludes")
        (range (start 6 16) (end 6 43))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
