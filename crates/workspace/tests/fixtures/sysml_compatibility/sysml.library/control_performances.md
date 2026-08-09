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
# EXPECTED
~~~
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'that'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'that'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'BooleanEvaluation'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'BooleanEvaluation'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'BooleanEvaluation'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'that'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'that'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'BooleanEvaluation'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'BooleanEvaluation'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'BooleanEvaluation'
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
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,KwRedefines,Ident,KwSubsets,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,KwRedefines,Ident,KwSubsets,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwRedefines,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwSuccession,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwInv,OpenCurly,Ident,OpenParen,CloseParen,EqEq,Ident,Arrow,Ident,OpenParen,CloseParen,CloseCurly,
CloseCurly,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwRedefines,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwSuccession,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwInv,OpenCurly,KwNot,Ident,OpenParen,CloseParen,EqEq,Ident,Arrow,Ident,OpenParen,CloseParen,CloseCurly,
CloseCurly,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwRedefines,Ident,Semicolon,
KwIn,KwRedefines,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwSuccession,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwInv,OpenCurly,KwNot,Ident,OpenParen,CloseParen,EqEq,Ident,Arrow,Ident,OpenParen,CloseParen,CloseCurly,
CloseCurly,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwSuccession,Ident,KwThen,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwBinding,Ident,KwOf,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwInv,OpenCurly,Ident,Arrow,Ident,OpenParen,CloseParen,EqEq,Ident,Arrow,Ident,OpenParen,CloseParen,Minus,DecimalValue,CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ControlPerformances'
    (documentation)
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'SequenceFunctions::size')
    (import_decl private 'SequenceFunctions::notEmpty')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensBefore')
    (import_decl private 'Occurrences::SelfSameLifeLink')
    (import_decl private 'Performances::Performance')
    (import_decl private 'Performances::BooleanEvaluation')
    (behavior_def
      (documentation)
      (feature_def 'outgoingHBLink' : 'HappensBefore' multiplicity
        (documentation)
        (feature_def end :>> 'earlierOccurrence' :> 'that')))
    (behavior_def
      (documentation)
      (feature_def 'incomingHBLink' : 'HappensBefore' multiplicity
        (documentation)
        (feature_def end :>> 'laterOccurrence' :> 'that')))
    (behavior_def
      (documentation)
      (feature_def in 'ifTest' : 'BooleanEvaluation' multiplicity))
    (behavior_def
      (documentation)
      (feature_def in :>> 'ifTest')
      (feature_def in 'thenClause' : 'Occurrence' multiplicity)
      (succession_def multiplicity
        (connector_end)
        (connector_end))
      (invariant_def
        (result_expr_member)))
    (behavior_def
      (documentation)
      (feature_def in :>> 'ifTest')
      (feature_def in 'elseClause' : 'Occurrence' multiplicity)
      (succession_def multiplicity
        (connector_end)
        (connector_end))
      (invariant_def
        (result_expr_member)))
    (behavior_def
      (documentation)
      (feature_def in :>> 'ifTest')
      (feature_def in :>> 'thenClause')
      (feature_def in 'elseClause' : 'Occurrence' multiplicity)
      (succession_def multiplicity
        (connector_end)
        (connector_end))
      (invariant_def
        (result_expr_member)))
    (behavior_def
      (documentation)
      (feature_def in 'whileTest' : 'BooleanEvaluation' multiplicity)
      (feature_def in 'body' : 'Occurrence' multiplicity)
      (feature_def in 'untilTest' : 'BooleanEvaluation' multiplicity)
      (step_def)
      (step_def)
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (succession_def
        (connector_end)
        (connector_end))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector 'loopBack'
        (connector_end)
        (connector_end))
      (invariant_def
        (result_expr_member)))))
~~~
# FORMAT
~~~sysml
standard library package ControlPerformances {
    doc /*
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
        doc /*
		 * A DecisionPerformance is a Performance that represents the selection of one of the Successions
		 * that have the DecisionPerforance behavior as their source. All such Successions must subset the 
		 * outgoingHBLink feature of the source DecisionPerformance. For each instance of DecisionPerformance, 
		 * the outgoingHBLink is an instance of exactly one of the Successions, ordering the DecisionPerformance
		 * as happening before an instance of the target of that Succcession.
		 */

        feature outgoingHBLink : HappensBefore [1] {
            doc /*
			 * Specializations subset this by all
			 * successions going out of a decision step.
			 */

            end feature redefines earlierOccurrence subsets that;
        }
    }

    behavior MergePerformance specializes Performance {
        doc /*
		 * A MergePerformance is a Performance that represents the merging of all Successions that
		 * target the MergePerforance behavior. All such Successions must subset the incomingHBLink
		 * feature of the target MergePerformance. For each instance of MergePerformance, the
		 * incomingHBLink is an instance of exactly one of the Successions, ordering the
		 * MergePerformance as happening after an instance of the source of that Succession.
		 */

        feature incomingHBLink : HappensBefore [1] {
            doc /*
			 * Specializations subset this by all
			 * successions coming into a merge step.
			 */

            end feature redefines laterOccurrence subsets that;
        }
    }

    abstract behavior IfPerformance specializes Performance {
        doc /*
		 * An IfPerformance is a Performance that determines whether the ifTest evaluation result is true 
		 * (by whether ifTrue has a value).
		 */

        in ifTest: BooleanEvaluation [1];
    }

    behavior IfThenPerformance specializes IfPerformance {
        doc /*
		 * An IfThenPerformance is an IfPerformance where the thenClause occurs after and only after the 
		 * ifTest evaluation result is true.
		 */

        in redefines ifTest;
        in thenClause: Occurrence [0..1];
        succession [1] ifTest then [0..1] thenClause;
        inv { ifTest() == thenClause->notEmpty() }
    }

    behavior IfElsePerformance specializes IfPerformance {
        doc /*
		 * An IfElsePerformance is an IfPerformance where the elseClause occurs after and only 
		 * after the ifTest evaluation result is not true.
		 */

        in redefines ifTest;
        in elseClause: Occurrence [0..1];
        succession [1] ifTest then [0..1] elseClause;
        inv { not ifTest() == elseClause->notEmpty() }
    }

    behavior IfThenElsePerformance specializes IfThenPerformance {
        doc /*
		 * An IfThenElsePerformance is an IfThenPerformance with an additional elseClause that
		 * occurs after and only after the ifTest evaluation is false.
		 */

        in redefines ifTest;
        in redefines thenClause;
        in elseClause: Occurrence [0..1];
        succession [1] ifTest then [0..1] elseClause;
        inv { not ifTest() == elseClause->notEmpty() }
    }

    behavior LoopPerformance specializes Performance {
        doc /*
		 * A LoopPerformance is a Performance where the body occurs repeatedly in sequence (iterates) 
		 * as long as the whileTest evaluation result is true before each iteration (and after the 
		 * previous one, except the first time) and the untilTest evaluation result is not true after 
		 * each iteration and before the next one (except the last one).
		 */

        in whileTest: BooleanEvaluation [1..*];
        in body: Occurrence [0..*];
        in untilTest: BooleanEvaluation [0..*];

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
# SMG
~~~
(model
  (namespace
    (library_package 'ControlPerformances'
      (documentation)
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'SequenceFunctions::size'[unresolved])
      (membership_import private -> 'SequenceFunctions::notEmpty'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensBefore'[unresolved])
      (membership_import private -> 'Occurrences::SelfSameLifeLink'[unresolved])
      (membership_import private -> 'Performances::Performance'[unresolved])
      (membership_import private -> 'Performances::BooleanEvaluation'[unresolved])
      (behavior_def 'DecisionPerformance' :> 'Performance'[unresolved]
        (documentation)
        (feature_def 'outgoingHBLink' : 'HappensBefore'[unresolved]
          (multiplicity_range [1])
          (documentation)
          (feature_def end :>> 'earlierOccurrence'[unresolved] :> 'that'[unresolved])))
      (behavior_def 'MergePerformance' :> 'Performance'[unresolved]
        (documentation)
        (feature_def 'incomingHBLink' : 'HappensBefore'[unresolved]
          (multiplicity_range [1])
          (documentation)
          (feature_def end :>> 'laterOccurrence'[unresolved] :> 'that'[unresolved])))
      (behavior_def abstract 'IfPerformance' :> 'Performance'[unresolved]
        (documentation)
        (feature_def in 'ifTest' : 'BooleanEvaluation'[unresolved]
          (multiplicity_range [1])))
      (behavior_def 'IfThenPerformance' :> 'ControlPerformances::IfPerformance'[behavior_def]
        (documentation)
        (feature_def in :>> 'ControlPerformances::IfPerformance::ifTest'[feature_def])
        (feature_def in 'thenClause' : 'Occurrence'[unresolved]
          (multiplicity_range [0..1]))
        (succession_def
          (multiplicity_range [1])
          (connector_end 'ifTest')
          (connector_end 'thenClause'))
        (invariant_def
          (result_expr_membership)))
      (behavior_def 'IfElsePerformance' :> 'ControlPerformances::IfPerformance'[behavior_def]
        (documentation)
        (feature_def in :>> 'ControlPerformances::IfPerformance::ifTest'[feature_def])
        (feature_def in 'elseClause' : 'Occurrence'[unresolved]
          (multiplicity_range [0..1]))
        (succession_def
          (multiplicity_range [1])
          (connector_end 'ifTest')
          (connector_end 'elseClause'))
        (invariant_def
          (result_expr_membership)))
      (behavior_def 'IfThenElsePerformance' :> 'ControlPerformances::IfThenPerformance'[behavior_def]
        (documentation)
        (feature_def in :>> ''[feature_def])
        (feature_def in :>> 'ControlPerformances::IfThenPerformance::thenClause'[feature_def])
        (feature_def in 'elseClause' : 'Occurrence'[unresolved]
          (multiplicity_range [0..1]))
        (succession_def
          (multiplicity_range [1])
          (connector_end 'ifTest')
          (connector_end 'elseClause'))
        (invariant_def
          (result_expr_membership)))
      (behavior_def 'LoopPerformance' :> 'Performance'[unresolved]
        (documentation)
        (feature_def in 'whileTest' : 'BooleanEvaluation'[unresolved]
          (multiplicity_range [1..*]))
        (feature_def in 'body' : 'Occurrence'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'untilTest' : 'BooleanEvaluation'[unresolved]
          (multiplicity_range [0..*]))
        (step_def 'whileDecision' : 'ControlPerformances::IfThenPerformance'[behavior_def]
          (multiplicity_range [1..*]))
        (step_def 'untilDecision' : 'ControlPerformances::IfElsePerformance'[behavior_def]
          (multiplicity_range [0..*]))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'whileDecision.ifTest')
          (connector_end 'whileTest'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'whileDecision.thenClause')
          (connector_end 'body'))
        (succession_def
          (connector_end 'body')
          (connector_end 'untilDecision'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'untilDecision.ifTest')
          (connector_end 'untilTest'))
        (binding_connector_def 'loopBack'
          (connector_end 'untilDecision.elseClause')
          (connector_end 'whileDecision'))
        (invariant_def
          (result_expr_membership))))))
~~~
