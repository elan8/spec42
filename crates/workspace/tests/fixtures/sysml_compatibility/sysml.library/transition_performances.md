# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/TransitionPerformances
type=file
~~~
# SOURCE
~~~kerml
standard library package TransitionPerformances {
	doc
	/*
	 * This package contains a library model of the semantics of conditional transitions between occurrences, 
	 * including the performance of specified Behaviors when the transition occurs.
	 */

	private import ScalarValues::Boolean;
	private import ScalarValues::Natural;
	private import SequenceFunctions::isEmpty;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensBefore;
	private import Performances::Performance;
	private import Performances::Evaluation;
	private import Transfers::MessageTransfer;
	private import Transfers::AcceptPerformance;
	private import Transfers::acceptPerformances;
	private import ControlFunctions::allTrue;
	private import SequenceFunctions::size;
	
	abstract behavior TransitionPerformance {
		in feature transitionLinkSource: Performance[1];
		
		feature trigger: MessageTransfer[*];
		bool guard[*] subsets enclosedPerformances;
		step effect[*] subsets enclosedPerformances;

		feature triggerTarget : Occurrence [1] default this;
		feature transitionLink: HappensBefore[0..1];
		
		private binding [0..1] transitionLink.earlierOccurrence = [1] transitionLinkSource;
		private succession [1] transitionLinkSource then [*] effect;
		private succession [*] effect then [1] transitionLink.laterOccurrence;
		
		private connector [0..1] transitionLink to [1..*] trigger;
		private connector all guardConstraint: TPCGuardConstraint[*] 
			from [0..1] transitionLink to [*] guard;
			
		private succession all [*] trigger then [*] guard;
		private succession all [*] guard then [*] effect;

		feature accNum: Natural [1] = if isEmpty(trigger) ? 0 else 1;
		step 'accept': AcceptPerformance[accNum] subsets timeEnclosedOccurrences, acceptPerformances {
			feature redefines acceptedTransfer = trigger;
		}
        binding 'accept'.receiver = triggerTarget;

		private succession [*] guard then [accNum] 'accept';
	}
	
	behavior NonStateTransitionPerformance specializes TransitionPerformance {
		feature isTriggerAfter: Boolean default true;
		private succession [1] transitionLinkSource then [1] Performance::self;
		private feature taNum: Natural [1] = if isTriggerAfter ? size(trigger) else 0;
		private succession triggerAfter [taNum] first [0..1] transitionLinkSource then [*] trigger.endShot;
				
		private succession all [*] guard then [0..1] transitionLink.laterOccurrence;
	}
	
	assoc struct TPCGuardConstraint {
		end guardedLink [0..1] feature constrainedHBLink: HappensBefore;
		end 'bool' constrainedGuard;
		
		private inv { allTrue(constrainedGuard()) }
	}	
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'MessageTransfer'
semantic.unresolved_name 'enclosedPerformances'
semantic.unresolved_name 'enclosedPerformances'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'AcceptPerformance'
semantic.unresolved_name 'timeEnclosedOccurrences'
semantic.unresolved_name 'acceptPerformances'
semantic.unresolved_name 'acceptedTransfer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'HappensBefore'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'MessageTransfer'
semantic.unresolved_name 'enclosedPerformances'
semantic.unresolved_name 'enclosedPerformances'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'AcceptPerformance'
semantic.unresolved_name 'timeEnclosedOccurrences'
semantic.unresolved_name 'acceptPerformances'
semantic.unresolved_name 'acceptedTransfer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'HappensBefore'
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
KwAbstract,KwBehavior,Ident,OpenCurly,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwBool,Ident,OpenSquare,Star,CloseSquare,KwSubsets,Ident,Semicolon,
KwStep,Ident,OpenSquare,Star,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwBinding,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwPrivate,KwSuccession,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,Star,CloseSquare,Ident,Semicolon,
KwPrivate,KwSuccession,OpenSquare,Star,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Semicolon,
KwPrivate,KwConnector,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,Semicolon,
KwPrivate,KwConnector,KwAll,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,
KwFrom,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,Star,CloseSquare,Ident,Semicolon,
KwPrivate,KwSuccession,KwAll,OpenSquare,Star,CloseSquare,Ident,KwThen,OpenSquare,Star,CloseSquare,Ident,Semicolon,
KwPrivate,KwSuccession,KwAll,OpenSquare,Star,CloseSquare,Ident,KwThen,OpenSquare,Star,CloseSquare,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,KwIf,Ident,OpenParen,Ident,CloseParen,Question,DecimalValue,KwElse,DecimalValue,Semicolon,
KwStep,UnrestrictedName,Colon,Ident,OpenSquare,Ident,CloseSquare,KwSubsets,Ident,Comma,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwBinding,UnrestrictedName,Dot,Ident,Eq,Ident,Semicolon,
KwPrivate,KwSuccession,OpenSquare,Star,CloseSquare,Ident,KwThen,OpenSquare,Ident,CloseSquare,UnrestrictedName,Semicolon,
CloseCurly,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,KwDefault,KwTrue,Semicolon,
KwPrivate,KwSuccession,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,KwIf,Ident,Question,Ident,OpenParen,Ident,CloseParen,KwElse,DecimalValue,Semicolon,
KwPrivate,KwSuccession,Ident,OpenSquare,Ident,CloseSquare,KwFirst,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,Star,CloseSquare,Ident,Dot,Ident,Semicolon,
KwPrivate,KwSuccession,KwAll,OpenSquare,Star,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAssoc,KwStruct,Ident,OpenCurly,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,Semicolon,
KwEnd,UnrestrictedName,Ident,Semicolon,
KwPrivate,KwInv,OpenCurly,Ident,OpenParen,Ident,OpenParen,CloseParen,CloseParen,CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'TransitionPerformances'
    (documentation)
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'ScalarValues::Natural')
    (import_decl private 'SequenceFunctions::isEmpty')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensBefore')
    (import_decl private 'Performances::Performance')
    (import_decl private 'Performances::Evaluation')
    (import_decl private 'Transfers::MessageTransfer')
    (import_decl private 'Transfers::AcceptPerformance')
    (import_decl private 'Transfers::acceptPerformances')
    (import_decl private 'ControlFunctions::allTrue')
    (import_decl private 'SequenceFunctions::size')
    (behavior_def
      (feature_def in 'transitionLinkSource' : 'Performance' multiplicity)
      (feature_def 'trigger' : 'MessageTransfer' multiplicity)
      (boolean_expr_def)
      (step_def)
      (feature_def 'triggerTarget' : 'Occurrence' multiplicity value)
      (feature_def 'transitionLink' : 'HappensBefore' multiplicity)
      (binding_connector private multiplicity
        (connector_end)
        (connector_end))
      (succession_def private multiplicity
        (connector_end)
        (connector_end))
      (succession_def private multiplicity
        (connector_end)
        (connector_end))
      (connector_def private multiplicity
        (connector_end)
        (connector_end))
      (connector_def private 'guardConstraint' : 'TPCGuardConstraint' multiplicity
        (connector_end)
        (connector_end))
      (succession_def private multiplicity
        (connector_end)
        (connector_end))
      (succession_def private multiplicity
        (connector_end)
        (connector_end))
      (feature_def 'accNum' : 'Natural' multiplicity value)
      (step_def
        (feature_def :>> 'acceptedTransfer' value))
      (binding_connector
        (connector_end)
        (connector_end))
      (succession_def private multiplicity
        (connector_end)
        (connector_end)))
    (behavior_def
      (feature_def 'isTriggerAfter' : 'Boolean' value)
      (succession_def private multiplicity
        (connector_end)
        (connector_end))
      (feature_def private 'taNum' : 'Natural' multiplicity value)
      (succession_def private 'triggerAfter' multiplicity
        (connector_end)
        (connector_end))
      (succession_def private multiplicity
        (connector_end)
        (connector_end)))
    (assoc_struct_def 'TPCGuardConstraint'
      (feature_def end 'constrainedHBLink' multiplicity : 'HappensBefore')
      (malformed)
      (invariant_def
        (result_expr_member)))))
~~~
# FORMAT
~~~sysml
standard library package TransitionPerformances {
	doc
	/*
	 * This package contains a library model of the semantics of conditional transitions between occurrences, 
	 * including the performance of specified Behaviors when the transition occurs.
	 */

	private import ScalarValues::Boolean;
	private import ScalarValues::Natural;
	private import SequenceFunctions::isEmpty;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensBefore;
	private import Performances::Performance;
	private import Performances::Evaluation;
	private import Transfers::MessageTransfer;
	private import Transfers::AcceptPerformance;
	private import Transfers::acceptPerformances;
	private import ControlFunctions::allTrue;
	private import SequenceFunctions::size;
	
	abstract behavior TransitionPerformance {
		in feature transitionLinkSource: Performance[1];
		
		feature trigger: MessageTransfer[*];
		bool guard[*] subsets enclosedPerformances;
		step effect[*] subsets enclosedPerformances;

		feature triggerTarget : Occurrence [1] default this;
		feature transitionLink: HappensBefore[0..1];
		
		private binding [0..1] transitionLink.earlierOccurrence = [1] transitionLinkSource;
		private succession [1] transitionLinkSource then [*] effect;
		private succession [*] effect then [1] transitionLink.laterOccurrence;
		
		private connector [0..1] transitionLink to [1..*] trigger;
		private connector all guardConstraint: TPCGuardConstraint[*] 
			from [0..1] transitionLink to [*] guard;
			
		private succession all [*] trigger then [*] guard;
		private succession all [*] guard then [*] effect;

		feature accNum: Natural [1] = if isEmpty(trigger) ? 0 else 1;
		step 'accept': AcceptPerformance[accNum] subsets timeEnclosedOccurrences, acceptPerformances {
			feature redefines acceptedTransfer = trigger;
		}
        binding 'accept'.receiver = triggerTarget;

		private succession [*] guard then [accNum] 'accept';
	}
	
	behavior NonStateTransitionPerformance specializes TransitionPerformance {
		feature isTriggerAfter: Boolean default true;
		private succession [1] transitionLinkSource then [1] Performance::self;
		private feature taNum: Natural [1] = if isTriggerAfter ? size(trigger) else 0;
		private succession triggerAfter [taNum] first [0..1] transitionLinkSource then [*] trigger.endShot;
				
		private succession all [*] guard then [0..1] transitionLink.laterOccurrence;
	}
	
	assoc struct TPCGuardConstraint {
		end guardedLink [0..1] feature constrainedHBLink: HappensBefore;
		end 'bool' constrainedGuard;
		
		private inv { allTrue(constrainedGuard()) }
	}	
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "TransitionPerformances"))) (name "TransitionPerformances") (declared-name "TransitionPerformances")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "TransitionPerformances::AcceptPerformance"))) (name "AcceptPerformance") (declared-name "AcceptPerformance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TransitionPerformances::Boolean"))) (name "Boolean") (declared-name "Boolean"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TransitionPerformances::Evaluation"))) (name "Evaluation") (declared-name "Evaluation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TransitionPerformances::HappensBefore"))) (name "HappensBefore") (declared-name "HappensBefore"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TransitionPerformances::MessageTransfer"))) (name "MessageTransfer") (declared-name "MessageTransfer"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TransitionPerformances::Natural"))) (name "Natural") (declared-name "Natural"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TransitionPerformances::NonStateTransitionPerformance"))) (name "NonStateTransitionPerformance") (declared-name "NonStateTransitionPerformance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TransitionPerformances::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TransitionPerformances::Performance"))) (name "Performance") (declared-name "Performance"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TransitionPerformances::TransitionPerformance"))) (name "TransitionPerformance") (declared-name "TransitionPerformance"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "TransitionPerformances::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "TransitionPerformances::acceptPerformances"))) (name "acceptPerformances") (declared-name "acceptPerformances"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TransitionPerformances::allTrue"))) (name "allTrue") (declared-name "allTrue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TransitionPerformances::isEmpty"))) (name "isEmpty") (declared-name "isEmpty"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TransitionPerformances::size"))) (name "size") (declared-name "size"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TransitionPerformances::struct"))) (name "struct") (declared-name "struct"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TransitionPerformances::_documentation"))) (to (node (document "d0") (qualified-name "TransitionPerformances"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
