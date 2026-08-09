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
# EXPECTED
~~~
semantic.unresolved_name 'DecisionPerformance'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'MessageTransfer'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'self'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'isRunToCompletion'
semantic.unresolved_name 'runToCompletionScope'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'TransitionPerformance'
semantic.unresolved_name 'TransitionPerformance'
semantic.unresolved_name 'TransitionPerformance'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'TransitionPerformance::transitionLinkSource'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'triggerTarget::incomingTransfersToSelf'
semantic.unresolved_name 'TransitionPerformance::trigger'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'HappensDuring'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'DecisionPerformance'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'MessageTransfer'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'self'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'isRunToCompletion'
semantic.unresolved_name 'runToCompletionScope'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'TransitionPerformance'
semantic.unresolved_name 'TransitionPerformance'
semantic.unresolved_name 'TransitionPerformance'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'TransitionPerformance::transitionLinkSource'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'triggerTarget::incomingTransfersToSelf'
semantic.unresolved_name 'TransitionPerformance::trigger'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'HappensDuring'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,KwDefault,KwTrue,Semicolon,
KwAbstract,KwStep,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwStep,KwEntry,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwStep,KwDo,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwStep,KwExit,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwStep,Ident,OpenSquare,Star,CloseSquare,KwSubsets,Ident,Eq,Ident,Arrow,Ident,OpenParen,KwDo,CloseParen,Semicolon,
KwPrivate,KwSuccession,OpenSquare,DecimalValue,CloseSquare,KwEntry,KwThen,OpenSquare,Star,CloseSquare,Ident,Semicolon,
KwPrivate,KwSuccession,OpenSquare,DecimalValue,CloseSquare,KwDo,Dot,Ident,KwThen,OpenSquare,Star,CloseSquare,Ident,Dot,Ident,Semicolon,
KwPrivate,KwSuccession,OpenSquare,Star,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,KwExit,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwDefault,KwNull,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPrivate,KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,KwAs,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwInv,OpenCurly,Ident,EqEq,Ident,Pipe,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseCurly,
KwInv,OpenCurly,Ident,KwImplies,
Ident,OpenParen,Ident,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
Ident,EqEq,Ident,Pipe,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Pipe,
Ident,OpenParen,Ident,Dot,KwExit,Dot,Ident,Dot,Ident,Comma,Ident,Dot,KwExit,Dot,Ident,CloseParen,Pipe,
OpenParen,Ident,Dot,Ident,BangEq,Ident,Ampersand,
OpenParen,Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,CloseParen,Pipe,
Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,CloseParen,CloseParen,CloseParen,CloseCurly,CloseCurly,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Eq,
Ident,Dot,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,Ident,KwIstype,Ident,CloseCurly,Semicolon,
KwReturn,Colon,Ident,OpenSquare,Star,CloseSquare,Eq,
Ident,OpenParen,Ident,Comma,
Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,Ident,OpenParen,Ident,CloseParen,CloseCurly,CloseParen,Semicolon,
CloseCurly,
KwPrivate,KwSuccession,OpenSquare,Star,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,KwExit,Semicolon,
KwFeature,KwRedefines,Ident,KwDefault,Ident,Dot,Ident,Semicolon,
KwFeature,KwRedefines,Ident,KwDefault,Ident,Dot,Ident,Semicolon,
KwInv,OpenCurly,Ident,KwImplies,
Ident,OpenParen,Ident,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
Ident,OpenParen,Ident,Dot,Ident,Comma,KwEntry,CloseParen,Pipe,Ident,OpenParen,Ident,Dot,Ident,Comma,KwEntry,CloseParen,CloseCurly,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Eq,
Ident,Dot,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
Ident,KwIstype,Ident,CloseCurly,Semicolon,
KwReturn,Colon,Ident,OpenSquare,Star,CloseSquare,Eq,
Ident,OpenParen,Ident,Comma,
Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
Ident,OpenParen,Ident,CloseParen,CloseCurly,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,OpenCurly,KwNot,Ident,Dot,Ident,Pipe,Ident,CloseCurly,
KwIn,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Semicolon,
KwFeature,KwRedefines,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPrivate,KwSuccession,OpenSquare,Star,CloseSquare,Ident,Dot,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Colon,Ident,Eq,Ident,Dot,Ident,OpenCurly,
KwInv,OpenCurly,OpenParen,Ident,KwIstype,Ident,CloseParen,KwImplies,
OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,EqEq,Ident,CloseCurly,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwSubsets,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Semicolon,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,KwSubsets,Ident,Comma,Ident,Dot,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Semicolon,
CloseCurly,
KwPrivate,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,KwIf,KwNot,Ident,Question,DecimalValue,KwElse,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwPrivate,KwConnector,Ident,Colon,Ident,OpenSquare,Ident,CloseSquare,KwFrom,OpenSquare,Star,CloseSquare,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwPrivate,KwSuccession,KwAll,OpenSquare,Star,CloseSquare,Ident,KwThen,OpenSquare,Star,CloseSquare,Ident,Semicolon,
KwPrivate,KwSuccession,OpenSquare,Star,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,KwExit,Semicolon,
KwPrivate,KwSuccession,OpenSquare,Ident,CloseSquare,KwAccept,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,KwExit,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'StatePerformances'
    (documentation)
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'ScalarValues::Natural')
    (import_decl private 'TransitionPerformances::TransitionPerformance')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensDuring')
    (import_decl private 'Transfers::Transfer')
    (import_decl private 'Transfers::MessageTransfer')
    (import_decl private 'Performances::Performance')
    (import_decl private 'ControlPerformances::DecisionPerformance')
    (import_decl private 'ControlFunctions::forAll')
    (import_decl private 'ControlFunctions::select')
    (import_decl private 'ControlFunctions::collect')
    (import_decl private 'SequenceFunctions::*')
    (behavior_def
      (feature_def 'isTriggerDuring' : 'Boolean' value)
      (step_def
        (documentation))
      (comment)
      (step_def)
      (step_def)
      (step_def)
      (step_def)
      (succession_def private multiplicity
        (connector_end)
        (connector_end))
      (succession_def private multiplicity
        (connector_end)
        (connector_end))
      (succession_def private multiplicity
        (connector_end)
        (connector_end))
      (feature_def 'incomingTransitionTrigger' : 'MessageTransfer' multiplicity value
        (documentation))
      (invariant_def
        (result_expr_member))
      (feature_def 'accepted' : 'Transfer' multiplicity :> 'acceptable'
        (documentation))
      (feature_def 'deferrable' : 'Transfer' multiplicity :> 'acceptable'
        (documentation))
      (feature_def abstract 'acceptable' : 'Transfer' multiplicity
        (documentation)
        (feature_def 'thatSP' : 'StatePerformance' multiplicity value)
        (feature_def 'accableT' : 'Transfer' :>> 'self')
        (feature_def 'accT' : 'Transfer' value)
        (invariant_def
          (result_expr_member))
        (invariant_def
          (result_expr_member)))
      (function_def
        (feature_def in 'p' : 'Performance' multiplicity)
        (feature_def 'substatePerformances' : 'StatePerformance' multiplicity value)
        (return_member))
      (succession_def private multiplicity
        (connector_end)
        (connector_end))
      (feature_def :>> 'isRunToCompletion' value)
      (feature_def :>> 'runToCompletionScope' value)
      (invariant_def
        (result_expr_member))
      (function_def
        (feature_def in 'p' : 'Performance' multiplicity)
        (feature_def 'subtransitionPerformances' : 'TransitionPerformance' multiplicity value)
        (return_member)))
    (behavior_def
      (feature_def 'isTriggerDuring' : 'Boolean' multiplicity)
      (invariant_def
        (result_expr_member))
      (feature_def in 'transitionLinkSource' : 'StatePerformance' :>> 'TransitionPerformance::transitionLinkSource'
        (feature_def :>> 'accepted')
        (feature_def :>> 'StatePerformance::acceptable'))
      (succession_def private multiplicity
        (connector_end)
        (connector_end))
      (feature_def private 'transitionLinkTarget' multiplicity : 'Occurrence' value
        (invariant_def
          (result_expr_member)))
      (feature_def 'acceptable' : 'Transfer' multiplicity :> 'transitionLinkSource.acceptable', 'triggerTarget.incomingTransfersToSelf')
      (feature_def 'trigger' :>> 'TransitionPerformance::trigger' :> 'acceptable', 'transitionLinkSource.accepted'
        (feature_def :>> 'endShot'))
      (feature_def private 'tdNum' : 'Natural' multiplicity value)
      (connector_def private 'linkTriggerDuring' : 'HappensDuring' multiplicity
        (connector_end)
        (connector_end))
      (succession_def private multiplicity
        (connector_end)
        (connector_end))
      (succession_def private multiplicity
        (connector_end)
        (connector_end))
      (succession_def private multiplicity
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
standard library package StatePerformances {
    doc /*
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
        feature isTriggerDuring : Boolean default = true;

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

        feature incomingTransitionTrigger : MessageTransfer [0..1] default = null {
            doc /*
			 * Transfer that triggered a transition into this state performance. 
			 */
        }

        private inv { isEmpty(accepted) == isEmpty(acceptable) }
        feature accepted : Transfer [0..1] subsets acceptable {
            doc /* A transfer to the trigger target of an outgoing transition performance
			 * for an outgoing successon that is taken for this state occurrence.
			 */
        }
        feature deferrable : Transfer [0..*] subsets acceptable {
            doc /* Transfers to trigger targets of outgoing transition performances can be
			 * considered for acceptance more than once.
			 */
        }
        abstract feature acceptable : Transfer [*] {
            doc /*
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

        feature redefines isRunToCompletion default = this.isRunToCompletion;
        feature redefines runToCompletionScope default = this.runToCompletionScope;
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
        feature isTriggerDuring : Boolean [1];
        inv { not transitionLinkSource.isTriggerDuring | isTriggerDuring  }

        in feature transitionLinkSource : StatePerformance redefines TransitionPerformance::transitionLinkSource {
            feature redefines accepted;
            feature redefines StatePerformance::acceptable;
        }
        private succession [*] transitionLinkSource.nonDoMiddle then [1] Performance::self;

        private feature transitionLinkTarget[0..1] : Occurrence = transitionLink.laterOccurrence {
            inv { (that istype StatePerformance) implies
			      (that as StatePerformance).incomingTransitionTrigger == trigger }
        }

        feature acceptable : Transfer [*] subsets transitionLinkSource.acceptable, triggerTarget.incomingTransfersToSelf;

        feature trigger redefines TransitionPerformance::trigger subsets acceptable, transitionLinkSource.accepted {
            feature redefines endShot;
        }

        private feature tdNum : Natural [1] = if not isTriggerDuring ? 0 else size(trigger);
        private connector linkTriggerDuring : HappensDuring [tdNum] from [*] trigger.endShot to [0..1] transitionLinkSource;

        private succession all [*] acceptable then [*] guard;
        private succession [*] guard then [1] transitionLinkSource.exit;

        private succession [accNum] accept then [1] transitionLinkSource.exit;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'StatePerformances'
      (documentation)
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'ScalarValues::Natural'[unresolved])
      (membership_import private -> 'TransitionPerformances::TransitionPerformance'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensDuring'[unresolved])
      (membership_import private -> 'Transfers::Transfer'[unresolved])
      (membership_import private -> 'Transfers::MessageTransfer'[unresolved])
      (membership_import private -> 'Performances::Performance'[unresolved])
      (membership_import private -> 'ControlPerformances::DecisionPerformance'[unresolved])
      (membership_import private -> 'ControlFunctions::forAll'[unresolved])
      (membership_import private -> 'ControlFunctions::select'[unresolved])
      (membership_import private -> 'ControlFunctions::collect'[unresolved])
      (namespace_import private -> 'SequenceFunctions'[unresolved])
      (behavior_def 'StatePerformance' :> 'DecisionPerformance'[unresolved]
        (feature_def 'isTriggerDuring' : 'Boolean'[unresolved]
          (feature_value (default =)))
        (step_def abstract 'middle'
          (multiplicity_range [1..*])
          (documentation))
        (step_def 'entry'
          (multiplicity_range [1]))
        (step_def 'do' :> 'StatePerformances::StatePerformance::middle'[step_def]
          (multiplicity_range [1]))
        (step_def 'exit'
          (multiplicity_range [1]))
        (step_def 'nonDoMiddle' :> 'StatePerformances::StatePerformance::middle'[step_def]
          (multiplicity_range [*])
          (feature_value (=)))
        (succession_def
          (multiplicity_range [1])
          (connector_end 'entry')
          (connector_end 'middle'))
        (succession_def
          (multiplicity_range [1])
          (connector_end 'do.startShot')
          (connector_end 'nonDoMiddle.startShot'))
        (succession_def
          (multiplicity_range [*])
          (connector_end 'middle')
          (connector_end 'exit'))
        (feature_def 'incomingTransitionTrigger' : 'MessageTransfer'[unresolved]
          (multiplicity_range [0..1])
          (feature_value (default =))
          (documentation))
        (invariant_def
          (result_expr_membership))
        (feature_def 'accepted' : 'Transfer'[unresolved] :> 'StatePerformances::StatePerformance::acceptable'[feature_def]
          (multiplicity_range [0..1])
          (documentation))
        (feature_def 'deferrable' : 'Transfer'[unresolved] :> 'StatePerformances::StatePerformance::acceptable'[feature_def]
          (multiplicity_range [0..*])
          (documentation))
        (feature_def abstract 'acceptable' : 'Transfer'[unresolved]
          (multiplicity_range [*])
          (documentation)
          (feature_def 'thatSP' : 'StatePerformances::StatePerformance'[behavior_def]
            (multiplicity_range [1])
            (feature_value (=)))
          (feature_def 'accableT' : 'Transfer'[unresolved] :>> 'self'[unresolved])
          (feature_def 'accT' : 'Transfer'[unresolved]
            (feature_value (=)))
          (invariant_def
            (result_expr_membership))
          (invariant_def
            (result_expr_membership)))
        (function_def 'allSubstatePerformances'
          (feature_def in 'p' : 'Performance'[unresolved]
            (multiplicity_range [1]))
          (feature_def 'substatePerformances' : 'StatePerformances::StatePerformance'[behavior_def]
            (multiplicity_range [*])
            (feature_value (=)))
          (return_parameter_membership
            (feature_def out : 'StatePerformances::StatePerformance'[behavior_def]
              (multiplicity_range [*])
              (feature_value (=)))))
        (succession_def
          (multiplicity_range [*])
          (connector_end 'acceptable')
          (connector_end 'exit'))
        (feature_def :>> 'isRunToCompletion'[unresolved]
          (feature_value (default =)))
        (feature_def :>> 'runToCompletionScope'[unresolved]
          (feature_value (default =)))
        (invariant_def
          (result_expr_membership))
        (function_def 'allSubtransitionPerformances'
          (feature_def in 'p' : 'Performance'[unresolved]
            (multiplicity_range [1]))
          (feature_def 'subtransitionPerformances' : 'TransitionPerformance'[unresolved]
            (multiplicity_range [*])
            (feature_value (=)))
          (return_parameter_membership
            (feature_def out : 'TransitionPerformance'[unresolved]
              (multiplicity_range [*])
              (feature_value (=))))))
      (behavior_def 'StateTransitionPerformance' :> 'TransitionPerformance'[unresolved]
        (feature_def 'isTriggerDuring' : 'Boolean'[unresolved]
          (multiplicity_range [1]))
        (invariant_def
          (result_expr_membership))
        (feature_def in 'transitionLinkSource' : 'StatePerformances::StatePerformance'[behavior_def] :>> 'TransitionPerformance::transitionLinkSource'[unresolved]
          (feature_def :>> 'StatePerformances::StatePerformance::accepted'[feature_def])
          (feature_def :>> 'StatePerformances::StatePerformance::acceptable'[feature_def]))
        (succession_def
          (multiplicity_range [*])
          (connector_end 'transitionLinkSource.nonDoMiddle')
          (connector_end 'Performance::self'))
        (feature_def 'transitionLinkTarget' : 'Occurrence'[unresolved]
          (multiplicity_range [0..1])
          (feature_value (=))
          (invariant_def
            (result_expr_membership)))
        (feature_def 'acceptable' : 'Transfer'[unresolved] :> ''[feature_def] :> 'triggerTarget::incomingTransfersToSelf'[unresolved]
          (multiplicity_range [*]))
        (feature_def 'trigger' :>> 'TransitionPerformance::trigger'[unresolved] :> 'StatePerformances::StateTransitionPerformance::acceptable'[feature_def] :> ''[feature_def]
          (feature_def :>> 'endShot'[unresolved]))
        (feature_def 'tdNum' : 'Natural'[unresolved]
          (multiplicity_range [1])
          (feature_value (=)))
        (connector_def 'linkTriggerDuring' : 'HappensDuring'[unresolved]
          (multiplicity_range [?])
          (connector_end 'trigger.endShot')
          (connector_end 'transitionLinkSource'))
        (succession_def
          (multiplicity_range [*])
          (connector_end 'acceptable')
          (connector_end 'guard'))
        (succession_def
          (multiplicity_range [*])
          (connector_end 'guard')
          (connector_end 'transitionLinkSource.exit'))
        (succession_def
          (multiplicity_range [?])
          (connector_end 'accept')
          (connector_end 'transitionLinkSource.exit'))))))
~~~
