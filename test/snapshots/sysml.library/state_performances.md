# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/StatePerformances
type=file
~~~
# SOURCE
~~~kerml
standard library package StatePerformances {
	doc
	/*
	 * This package contains a library model of the semantics of state-based behavior,
	 * including the performance of (behavioral) states and the transitions between them.
	 */

	private import ScalarValues::Boolean;
	private import ScalarValues::Natural;
	private import TransitionPerformances::TransitionPerformance;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensDuring;
	private import Transfers::Transfer;
	private import Transfers::MessageTransfer;
	private import Performances::Performance;
	private import ControlPerformances::DecisionPerformance;
	private import ControlFunctions::forAll;
	private import ControlFunctions::select;
	private import ControlFunctions::collect;
	private import SequenceFunctions::*;
	
	behavior StatePerformance specializes DecisionPerformance {
		feature isTriggerDuring: Boolean default true;

		abstract step middle[1..*] {
			doc
			/*
			 * All modeler-defined steps must subset this.
			 */
		}
		
		/* 
		 * Note: All steps are implicitly considered to be enclosedPerformances,
		 * and hence happening during the state performance.
		 */
		
		step entry[1];
		step do[1] subsets middle;
		step exit[1];
		
		step nonDoMiddle[*] subsets middle = middle->excluding(do);

		private succession [1] entry then [*] middle;
		private succession [1] do.startShot then [*] nonDoMiddle.startShot;
		private succession [*] middle then [1] exit;


		feature incomingTransitionTrigger : MessageTransfer [0..1] default null {
			doc
			/*
			 * Transfer that triggered a transition into this state performance. 
			 */
		}

		private inv { isEmpty(accepted) == isEmpty(acceptable) }
		feature accepted: Transfer[0..1] subsets acceptable {
			doc
			/* A transfer to the trigger target of an outgoing transition performance
			 * for an outgoing successon that is taken for this state occurrence.
			 */
		}
		feature deferrable: Transfer[0..*] subsets acceptable {
			doc
			/* Transfers to trigger targets of outgoing transition performances can be
			 * considered for acceptance more than once.
			 */
		}
		abstract feature acceptable: Transfer[*] {
			doc
			/*
			 * Transfers that might be accepted. 
			 */
			feature thatSP : StatePerformance [1] = that as StatePerformance;
			feature accableT : Transfer redefines self;
			feature accT : Transfer = thatSP.accepted;
			inv { accableT == accT | incomingTransferSort(accT, accableT) }  
			inv { isDispatch implies
				     allSubstatePerformances(dispatchScope)->forAll{in oSP : StatePerformance;						
						  oSP == thatSP | isEmpty(oSP.accepted) |
	   					  includes(thatSP.exit.startShot.successors, oSP.exit.startShot) |
						  ( oSP.accepted != accableT & 
							( incomingTransferSort(oSP.accepted, accableT) |
							  includes(oSP.deferrable, accableT) ) ) } }
		}
		
		function allSubstatePerformances {
			in p : Performance [1];
			feature substatePerformances: StatePerformance [*] =
				p.subperformances->select{in subp:Performance; subp istype StatePerformance};
			return  : StatePerformance [*] =
				union(substatePerformances,  
					  substatePerformances->collect{in sp:StatePerformance; allSubstatePerformances(sp) } );
		}
		
		private succession [*] acceptable then [1] exit;
		
		feature redefines isRunToCompletion default this.isRunToCompletion;
		feature redefines runToCompletionScope default this.runToCompletionScope; 
		inv { isRunToCompletion implies
			     allSubtransitionPerformances(runToCompletionScope)->forAll{in tp : TransitionPerformance;
				    includes(tp.successors, entry) | includes(tp.predecessors, entry) }
		}
		
		function allSubtransitionPerformances {
			in p : Performance [1];
			feature subtransitionPerformances: TransitionPerformance [*] =
	 			 p.subperformances->select{in subp:Performance;
		 			   subp istype StateTransitionPerformance };
			return  : TransitionPerformance [*] =
				union(subtransitionPerformances,  
		      		subtransitionPerformances->collect{in sp:TransitionPerformance; 
							 		allSubtransitionPerformances(sp) } ); 
 		}
	}
	
	behavior StateTransitionPerformance specializes TransitionPerformance {
		feature isTriggerDuring: Boolean[1];
		inv { not transitionLinkSource.isTriggerDuring | isTriggerDuring  }

		in feature transitionLinkSource: StatePerformance redefines TransitionPerformance::transitionLinkSource {
			feature redefines accepted;
			feature redefines StatePerformance::acceptable;
		}
		private succession [*] transitionLinkSource.nonDoMiddle then [1] Performance::self;

		private feature transitionLinkTarget [0..1] : Occurrence = transitionLink.laterOccurrence {
			inv { (that istype StatePerformance) implies
			      (that as StatePerformance).incomingTransitionTrigger == trigger }
		}
		
		feature acceptable: Transfer [*] subsets transitionLinkSource.acceptable, triggerTarget.incomingTransfersToSelf;

		feature trigger redefines TransitionPerformance::trigger subsets acceptable, transitionLinkSource.accepted {
			feature redefines endShot;
		}
		
		private feature tdNum: Natural [1] = if not isTriggerDuring ? 0 else size(trigger);
		private connector linkTriggerDuring: HappensDuring[tdNum] from [*] trigger.endShot to [0..1] transitionLinkSource;
		
		private succession all [*] acceptable then [*] guard;
		private succession [*] guard then [1] transitionLinkSource.exit;

		private succession [accNum] accept then [1] transitionLinkSource.exit;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_performances.md"
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
        (range (start 8 16) (end 8 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 61))
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
        (range (start 12 16) (end 12 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 16) (end 18 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 33))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package StatePerformances {
	doc
	/*
	 * This package contains a library model of the semantics of state-based behavior,
	 * including the performance of (behavioral) states and the transitions between them.
	 */

	private import ScalarValues::Boolean;
	private import ScalarValues::Natural;
	private import TransitionPerformances::TransitionPerformance;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensDuring;
	private import Transfers::Transfer;
	private import Transfers::MessageTransfer;
	private import Performances::Performance;
	private import ControlPerformances::DecisionPerformance;
	private import ControlFunctions::forAll;
	private import ControlFunctions::select;
	private import ControlFunctions::collect;
	private import SequenceFunctions::*;
	
	behavior StatePerformance specializes DecisionPerformance {
		feature isTriggerDuring: Boolean default true;

		abstract step middle[1..*] {
			doc
			/*
			 * All modeler-defined steps must subset this.
			 */
		}
		
		/* 
		 * Note: All steps are implicitly considered to be enclosedPerformances,
		 * and hence happening during the state performance.
		 */
		
		step entry[1];
		step do[1] subsets middle;
		step exit[1];
		
		step nonDoMiddle[*] subsets middle = middle->excluding(do);

		private succession [1] entry then [*] middle;
		private succession [1] do.startShot then [*] nonDoMiddle.startShot;
		private succession [*] middle then [1] exit;


		feature incomingTransitionTrigger : MessageTransfer [0..1] default null {
			doc
			/*
			 * Transfer that triggered a transition into this state performance. 
			 */
		}

		private inv { isEmpty(accepted) == isEmpty(acceptable) }
		feature accepted: Transfer[0..1] subsets acceptable {
			doc
			/* A transfer to the trigger target of an outgoing transition performance
			 * for an outgoing successon that is taken for this state occurrence.
			 */
		}
		feature deferrable: Transfer[0..*] subsets acceptable {
			doc
			/* Transfers to trigger targets of outgoing transition performances can be
			 * considered for acceptance more than once.
			 */
		}
		abstract feature acceptable: Transfer[*] {
			doc
			/*
			 * Transfers that might be accepted. 
			 */
			feature thatSP : StatePerformance [1] = that as StatePerformance;
			feature accableT : Transfer redefines self;
			feature accT : Transfer = thatSP.accepted;
			inv { accableT == accT | incomingTransferSort(accT, accableT) }  
			inv { isDispatch implies
				     allSubstatePerformances(dispatchScope)->forAll{in oSP : StatePerformance;						
						  oSP == thatSP | isEmpty(oSP.accepted) |
	   					  includes(thatSP.exit.startShot.successors, oSP.exit.startShot) |
						  ( oSP.accepted != accableT & 
							( incomingTransferSort(oSP.accepted, accableT) |
							  includes(oSP.deferrable, accableT) ) ) } }
		}
		
		function allSubstatePerformances {
			in p : Performance [1];
			feature substatePerformances: StatePerformance [*] =
				p.subperformances->select{in subp:Performance; subp istype StatePerformance};
			return  : StatePerformance [*] =
				union(substatePerformances,  
					  substatePerformances->collect{in sp:StatePerformance; allSubstatePerformances(sp) } );
		}
		
		private succession [*] acceptable then [1] exit;
		
		feature redefines isRunToCompletion default this.isRunToCompletion;
		feature redefines runToCompletionScope default this.runToCompletionScope; 
		inv { isRunToCompletion implies
			     allSubtransitionPerformances(runToCompletionScope)->forAll{in tp : TransitionPerformance;
				    includes(tp.successors, entry) | includes(tp.predecessors, entry) }
		}
		
		function allSubtransitionPerformances {
			in p : Performance [1];
			feature subtransitionPerformances: TransitionPerformance [*] =
	 			 p.subperformances->select{in subp:Performance;
		 			   subp istype StateTransitionPerformance };
			return  : TransitionPerformance [*] =
				union(subtransitionPerformances,  
		      		subtransitionPerformances->collect{in sp:TransitionPerformance; 
							 		allSubtransitionPerformances(sp) } ); 
 		}
	}
	
	behavior StateTransitionPerformance specializes TransitionPerformance {
		feature isTriggerDuring: Boolean[1];
		inv { not transitionLinkSource.isTriggerDuring | isTriggerDuring  }

		in feature transitionLinkSource: StatePerformance redefines TransitionPerformance::transitionLinkSource {
			feature redefines accepted;
			feature redefines StatePerformance::acceptable;
		}
		private succession [*] transitionLinkSource.nonDoMiddle then [1] Performance::self;

		private feature transitionLinkTarget [0..1] : Occurrence = transitionLink.laterOccurrence {
			inv { (that istype StatePerformance) implies
			      (that as StatePerformance).incomingTransitionTrigger == trigger }
		}
		
		feature acceptable: Transfer [*] subsets transitionLinkSource.acceptable, triggerTarget.incomingTransfersToSelf;

		feature trigger redefines TransitionPerformance::trigger subsets acceptable, transitionLinkSource.accepted {
			feature redefines endShot;
		}
		
		private feature tdNum: Natural [1] = if not isTriggerDuring ? 0 else size(trigger);
		private connector linkTriggerDuring: HappensDuring[tdNum] from [*] trigger.endShot to [0..1] transitionLinkSource;
		
		private succession all [*] acceptable then [*] guard;
		private succession [*] guard then [1] transitionLinkSource.exit;

		private succession [accNum] accept then [1] transitionLinkSource.exit;
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "00f32fe64091541ac06ee57e84bcd45b8f2242a9257e7fe4652c76f3b9085ef5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "StatePerformances"))) (kind "package") (name "StatePerformances") (declared-name "StatePerformances") (range (start (line 0) (character 0)) (end (line 0) (character 5456))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 19) (character 1)) (end (line 19) (character 37))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 19) (character 16)) (end (line 19) (character 33))))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 7) (character 1)) (end (line 7) (character 38))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 37))))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::DecisionPerformance"))) (kind "import") (name "DecisionPerformance") (declared-name "DecisionPerformance") (range (start (line 15) (character 1)) (end (line 15) (character 57))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::DecisionPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 56))))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::HappensDuring"))) (kind "import") (name "HappensDuring") (declared-name "HappensDuring") (range (start (line 11) (character 1)) (end (line 11) (character 43))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensDuring") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 42))))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::MessageTransfer"))) (kind "import") (name "MessageTransfer") (declared-name "MessageTransfer") (range (start (line 13) (character 1)) (end (line 13) (character 43))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::MessageTransfer") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 42))))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (range (start (line 8) (character 1)) (end (line 8) (character 38))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 37))))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 10) (character 1)) (end (line 10) (character 40))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 39))))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::Performance"))) (kind "import") (name "Performance") (declared-name "Performance") (range (start (line 14) (character 1)) (end (line 14) (character 42))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Performance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 41))))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::StatePerformance"))) (kind "kermlDecl") (name "StatePerformance") (declared-name "StatePerformance") (range (start (line 21) (character 1)) (end (line 21) (character 3288))) (parent (node (document "d0") (qualified-name "StatePerformances"))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::StateTransitionPerformance"))) (kind "kermlDecl") (name "StateTransitionPerformance") (declared-name "StateTransitionPerformance") (range (start (line 115) (character 1)) (end (line 115) (character 1357))) (parent (node (document "d0") (qualified-name "StatePerformances"))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::Transfer"))) (kind "import") (name "Transfer") (declared-name "Transfer") (range (start (line 12) (character 1)) (end (line 12) (character 36))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::Transfer") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 35))))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::TransitionPerformance"))) (kind "import") (name "TransitionPerformance") (declared-name "TransitionPerformance") (range (start (line 9) (character 1)) (end (line 9) (character 62))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "TransitionPerformances::TransitionPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 61))))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 5456))) (parent (node (document "d0") (qualified-name "StatePerformances"))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::collect"))) (kind "import") (name "collect") (declared-name "collect") (range (start (line 18) (character 1)) (end (line 18) (character 42))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::collect") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 18) (character 16)) (end (line 18) (character 41))))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (range (start (line 16) (character 1)) (end (line 16) (character 41))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 40))))))
    (element (id (node (document "d0") (qualified-name "StatePerformances::select"))) (kind "import") (name "select") (declared-name "select") (range (start (line 17) (character 1)) (end (line 17) (character 41))) (parent (node (document "d0") (qualified-name "StatePerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::select") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 16)) (end (line 17) (character 40))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SequenceFunctions::*") (range (start (line 19) (character 16)) (end (line 19) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 7) (character 16)) (end (line 7) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::DecisionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::DecisionPerformance") (range (start (line 15) (character 16)) (end (line 15) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::HappensDuring"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensDuring") (range (start (line 11) (character 16)) (end (line 11) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::MessageTransfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::MessageTransfer") (range (start (line 13) (character 16)) (end (line 13) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 8) (character 16)) (end (line 8) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 10) (character 16)) (end (line 10) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::Performance"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Performance") (range (start (line 14) (character 16)) (end (line 14) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::Transfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::Transfer") (range (start (line 12) (character 16)) (end (line 12) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::TransitionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "TransitionPerformances::TransitionPerformance") (range (start (line 9) (character 16)) (end (line 9) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::collect"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::collect") (range (start (line 18) (character 16)) (end (line 18) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (range (start (line 16) (character 16)) (end (line 16) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StatePerformances::select"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::select") (range (start (line 17) (character 16)) (end (line 17) (character 40))) (outcome (status unresolved)))
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
    (query (range (start 19 16) (end 19 33)) (probe (position 19 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "SequenceFunctions::*")
        (range (start 19 16) (end 19 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 35)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::Transfer"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::Transfer")
        (range (start 12 16) (end 12 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 37)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 7 16) (end 7 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 37)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::Natural"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 8 16) (end 8 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 39)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 10 16) (end 10 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 40)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 16 16) (end 16 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 16) (end 17 40)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::select"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::select")
        (range (start 17 16) (end 17 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 41)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::Performance"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::Performance")
        (range (start 14 16) (end 14 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 16) (end 18 41)) (probe (position 18 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::collect"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::collect")
        (range (start 18 16) (end 18 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 42)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::HappensDuring"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensDuring")
        (range (start 11 16) (end 11 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 42)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::MessageTransfer"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::MessageTransfer")
        (range (start 13 16) (end 13 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 56)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::DecisionPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::DecisionPerformance")
        (range (start 15 16) (end 15 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 61)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "StatePerformances::TransitionPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "TransitionPerformances::TransitionPerformance")
        (range (start 9 16) (end 9 61))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
