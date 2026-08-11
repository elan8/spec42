# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/ControlPerformances
type=file
~~~
# SOURCE
~~~kerml
standard library package ControlPerformances {
	doc
	/*
	 * This package defines Behaviors to be used to type Steps that control the sequencing of performance
	 * of other Steps. 
	 */

	private import ScalarValues::Boolean;
	private import SequenceFunctions::size;
	private import SequenceFunctions::notEmpty;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensBefore;
	private import Occurrences::SelfSameLifeLink;
	private import Performances::Performance;
	private import Performances::BooleanEvaluation;
	
	behavior DecisionPerformance specializes Performance {
		doc
		/*
		 * A DecisionPerformance is a Performance that represents the selection of one of the Successions
		 * that have the DecisionPerforance behavior as their source. All such Successions must subset the 
		 * outgoingHBLink feature of the source DecisionPerformance. For each instance of DecisionPerformance, 
		 * the outgoingHBLink is an instance of exactly one of the Successions, ordering the DecisionPerformance
		 * as happening before an instance of the target of that Succcession.
		 */
		
		feature outgoingHBLink: HappensBefore[1] {
			doc
			/*
			 * Specializations subset this by all
			 * successions going out of a decision step.
			 */

			 end feature redefines earlierOccurrence subsets that;
		}
	}
	
	behavior MergePerformance specializes Performance {
		doc
		/*
		 * A MergePerformance is a Performance that represents the merging of all Successions that
		 * target the MergePerforance behavior. All such Successions must subset the incomingHBLink
		 * feature of the target MergePerformance. For each instance of MergePerformance, the
		 * incomingHBLink is an instance of exactly one of the Successions, ordering the
		 * MergePerformance as happening after an instance of the source of that Succession.
		 */

		feature incomingHBLink: HappensBefore[1] {
			doc
			/*
			 * Specializations subset this by all
			 * successions coming into a merge step.
			 */

			 end feature redefines laterOccurrence subsets that;
		}
	}

	abstract behavior IfPerformance specializes Performance {
		doc
		/*
		 * An IfPerformance is a Performance that determines whether the ifTest evaluation result is true 
		 * (by whether ifTrue has a value).
		 */	
		 
		in ifTest : BooleanEvaluation[1];
	}
	
	behavior IfThenPerformance specializes IfPerformance {
		doc
		/*
		 * An IfThenPerformance is an IfPerformance where the thenClause occurs after and only after the 
		 * ifTest evaluation result is true.
		 */
		
		in redefines ifTest;
		in thenClause : Occurrence[0..1];
		succession [1] ifTest then [0..1] thenClause;
		inv { ifTest() == thenClause->notEmpty() }
	}
	
	behavior IfElsePerformance specializes IfPerformance {
		doc
		/*
		 * An IfElsePerformance is an IfPerformance where the elseClause occurs after and only 
		 * after the ifTest evaluation result is not true.
		 */

		in redefines ifTest;
		in elseClause : Occurrence[0..1];
		succession [1] ifTest then [0..1] elseClause;
		inv { not ifTest() == elseClause->notEmpty() }
	}
	
	behavior IfThenElsePerformance specializes IfThenPerformance {
		doc
		/*
		 * An IfThenElsePerformance is an IfThenPerformance with an additional elseClause that
		 * occurs after and only after the ifTest evaluation is false.
		 */
		 
		in redefines ifTest;
		in redefines thenClause;
        in elseClause : Occurrence[0..1];
        succession [1] ifTest then [0..1] elseClause;
        inv { not ifTest() == elseClause->notEmpty() }
	}
	
	behavior LoopPerformance specializes Performance {
		doc
		/*
		 * A LoopPerformance is a Performance where the body occurs repeatedly in sequence (iterates) 
		 * as long as the whileTest evaluation result is true before each iteration (and after the 
		 * previous one, except the first time) and the untilTest evaluation result is not true after 
		 * each iteration and before the next one (except the last one).
		 */
		 
		in whileTest : BooleanEvaluation[1..*];
		in body : Occurrence[0..*];
        in untilTest : BooleanEvaluation[0..*];
		
		step whileDecision : IfThenPerformance[1..*];
		step untilDecision : IfElsePerformance[0..*];
		
		binding [1] whileDecision.ifTest = [1] whileTest;
		binding [1] whileDecision.thenClause = [1] body;
		
		succession body then untilDecision;
		
		binding [1] untilDecision.ifTest = [1] untilTest;
		binding loopBack of [0..1] untilDecision.elseClause = [1] whileDecision;
		
		inv { loopBack->size() == whileDecision->size() - 1 }
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "control_performances.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 37))
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
        (range (start 9 16) (end 9 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 47))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "906b6658dbe37b7a5307b0d18b33a67cc0a9ea5d8faa4c4918d8285a8fe3b196") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ControlPerformances"))) (kind "package") (name "ControlPerformances") (declared-name "ControlPerformances"))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (parent (node (document "d0") (qualified-name "ControlPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::BooleanEvaluation"))) (kind "import") (name "BooleanEvaluation") (declared-name "BooleanEvaluation") (parent (node (document "d0") (qualified-name "ControlPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::BooleanEvaluation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::DecisionPerformance"))) (kind "kermlDecl") (name "DecisionPerformance") (declared-name "DecisionPerformance") (parent (node (document "d0") (qualified-name "ControlPerformances"))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::HappensBefore"))) (kind "import") (name "HappensBefore") (declared-name "HappensBefore") (parent (node (document "d0") (qualified-name "ControlPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::IfElsePerformance"))) (kind "kermlDecl") (name "IfElsePerformance") (declared-name "IfElsePerformance") (parent (node (document "d0") (qualified-name "ControlPerformances"))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::IfPerformance"))) (kind "kermlDecl") (name "IfPerformance") (declared-name "IfPerformance") (parent (node (document "d0") (qualified-name "ControlPerformances"))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::IfThenElsePerformance"))) (kind "kermlDecl") (name "IfThenElsePerformance") (declared-name "IfThenElsePerformance") (parent (node (document "d0") (qualified-name "ControlPerformances"))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::IfThenPerformance"))) (kind "kermlDecl") (name "IfThenPerformance") (declared-name "IfThenPerformance") (parent (node (document "d0") (qualified-name "ControlPerformances"))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::LoopPerformance"))) (kind "kermlDecl") (name "LoopPerformance") (declared-name "LoopPerformance") (parent (node (document "d0") (qualified-name "ControlPerformances"))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::MergePerformance"))) (kind "kermlDecl") (name "MergePerformance") (declared-name "MergePerformance") (parent (node (document "d0") (qualified-name "ControlPerformances"))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "ControlPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::Performance"))) (kind "import") (name "Performance") (declared-name "Performance") (parent (node (document "d0") (qualified-name "ControlPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Performance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::SelfSameLifeLink"))) (kind "import") (name "SelfSameLifeLink") (declared-name "SelfSameLifeLink") (parent (node (document "d0") (qualified-name "ControlPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::SelfSameLifeLink") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ControlPerformances"))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::notEmpty"))) (kind "import") (name "notEmpty") (declared-name "notEmpty") (parent (node (document "d0") (qualified-name "ControlPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::notEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ControlPerformances::size"))) (kind "import") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "ControlPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ControlPerformances::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlPerformances::BooleanEvaluation"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::BooleanEvaluation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlPerformances::HappensBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlPerformances::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlPerformances::Performance"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Performance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlPerformances::SelfSameLifeLink"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::SelfSameLifeLink") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlPerformances::notEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::notEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlPerformances::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 16) (end 7 37)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "ControlPerformances::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 7 16) (end 7 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 39)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "ControlPerformances::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 8 16) (end 8 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 39)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "ControlPerformances::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 10 16) (end 10 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 41)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "ControlPerformances::Performance"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::Performance")
        (range (start 13 16) (end 13 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 42)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "ControlPerformances::HappensBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensBefore")
        (range (start 11 16) (end 11 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 43)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "ControlPerformances::notEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
        (range (start 9 16) (end 9 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 45)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "ControlPerformances::SelfSameLifeLink"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::SelfSameLifeLink")
        (range (start 12 16) (end 12 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 47)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "ControlPerformances::BooleanEvaluation"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::BooleanEvaluation")
        (range (start 14 16) (end 14 47))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
