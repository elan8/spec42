# META
~~~ini
description=Standard Library: Domain Libraries/Analysis/TradeStudies
type=file
~~~
# SOURCE
~~~sysml
standard library package TradeStudies {
	doc
	/*
	 * This package provides a simple framework for defining trade-off study analysis cases.
	 */

	private import Base::Anything;
	private import ScalarValues::*;
	private import ScalarFunctions::*;
	private import ControlFunctions::*;
	
	abstract calc def EvaluationFunction {
		doc
		/*
		 * An EvaluationFunction is a calculation that evaluates a TradeStudy alternative,
		 * producing a ScalarValue that can be comparted with the evaluation of other
		 * alternatives.
		 */
	
		in ref alternative : Anything {
			doc
			/*
			 * The alternative to be evaluated.
			 */
		} 
		
		return attribute result : ScalarValue[1] {
			doc
			/*
			 * A ScalarValue representing the evaluation of the given alternative.
			 */
		} 
	}
	
	abstract requirement def TradeStudyObjective {
		doc
		/*
		 * A TradeStudyObjective is the base definition for the objective of a TradeStudy.
		 * The requirement is to choose from a given set of alternatives the selectedAlternative
		 * for that has the best evaluation according to a given EvaluationFunction. What
		 * value is considered "best" is not defined in the abstract base definition but must be
		 * computed in any concrete specialization.
		 */
	
		subject selectedAlternative : Anything {
			doc
			/*
			 * The alternative that should be selected, as evaluated using the given 
			 * ObjectiveFunction.
			 */
		}
		
		in ref alternatives : Anything[1..*] {
			doc
			/*
			 * The alternatives being considered in the TradeStudy for which this TradeStudyObjective 
			 * is the objective.
			 */
		}
		
		in calc eval : EvaluationFunction {
			doc
			/*
			 * The EvaluationFunction to be used in evaluating the given alternatives.
			 */
		}
		
		attribute best : ScalarValue {
			doc
			/*
			 * Out of the evaluation results of all the given alternatives, the one that is considered 
			 * "best", in the sense that it is the value the selectedAlternative should have. This 
			 * value must be computed in any concrete specialization of TradeStudyObjective.
			 */
		}
				
		require constraint { eval(selectedAlternative) == best }
	}
	
	requirement def MinimizeObjective :> TradeStudyObjective {
		doc
		/*
		 * A MinimizeObjective is a TradeStudyObjective that requires that the 
		 * selectedAlternative have the minimum ObjectiveFunction value of all the
		 * given alternatives.
		 */
		 
		attribute :>> best = alternatives->minimize {
			doc
			/*
			 * For a MinimizeObjective, the best value is the minimum one.
			 */
		
			in x; eval(x)
		};
	}
	
	requirement def MaximizeObjective :> TradeStudyObjective {
		doc
		/*
		 * A MaximizeObjective is a TradeStudyObjective that requires that the 
		 * selectedAlternative have the maximum ObjectiveFunction value of all the
		 * given alternatives.
		 */
	
		attribute :>> best = alternatives->maximize {
			doc
			/*
			 * For a MinimizeObjective, the best value is the maximum one.
			 */
		
			in x; eval(x)
		};
	}
	
	abstract analysis def TradeStudy {
		doc
		/*
		 * A TradeStudy is an analysis case whose subject is a set of alternatives
		 * (at least one) and whose result is a selection of one of those alternatives.
		 * The alternatives are evaluated based on a given ObjectiveFunction and the
		 * selection is made such that it satisfies the objective of the TradeStudy
		 * (which must be a TradeStudyObjective).
		 */
	
		subject studyAlternatives : Anything[1..*] {
			doc
			/*
			 * The set of alternatives being considered in this TradeStudy. 
			 * 
			 * In a TradeStudy usage, bind this feature to the actual collection of
			 * alternatives to be considered.
			 */
		}
		
		abstract calc evaluationFunction : EvaluationFunction {
			doc
			/*
			 * The EvaluationFunction to be used to evaluate the alternatives.
			 * 
			 * In a TradeStudy usage, redefine this feature to provide the desired
			 * calculation (or bind it to a calculation usage that does so).
			 */
		}
		
		objective tradeStudyObjective : TradeStudyObjective {
			doc
			/*
			 * The objective of this TradeStudy.
			 * 
			 * Redefine this feature to give it a definition that is a concrete
			 * specialization of TradeStudyObjective. That can either be one of the
			 * specializations provided in this package, or a more specific user-
			 * defined one.
			 */
		
            subject :>> selectedAlternative;
			in ref :>> alternatives = studyAlternatives;
			in calc :>> eval = evaluationFunction;
		}
		
		return selectedAlternative : Anything = studyAlternatives->selectOne {in ref a {
			doc
			/*
			 * The alternative selected by this TradeStudy, which is the one that meets the
			 * requirement of the tradeStudyObjective.
			 */
		} tradeStudyObjective(selectedAlternative = a)};
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "trade_studies.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 2) (end 19 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 2) (end 67 326))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 161 2) (end 161 282))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package TradeStudies {
	doc
	/*
	 * This package provides a simple framework for defining trade-off study analysis cases.
	 */

	private import Base::Anything;
	private import ScalarValues::*;
	private import ScalarFunctions::*;
	private import ControlFunctions::*;
	
	abstract calc def EvaluationFunction {
		doc
		/*
		 * An EvaluationFunction is a calculation that evaluates a TradeStudy alternative,
		 * producing a ScalarValue that can be comparted with the evaluation of other
		 * alternatives.
		 */
	
		in ref alternative : Anything {
			doc
			/*
			 * The alternative to be evaluated.
			 */
		} 
		
		return attribute result : ScalarValue[1] {
			doc
			/*
			 * A ScalarValue representing the evaluation of the given alternative.
			 */
		} 
	}
	
	abstract requirement def TradeStudyObjective {
		doc
		/*
		 * A TradeStudyObjective is the base definition for the objective of a TradeStudy.
		 * The requirement is to choose from a given set of alternatives the selectedAlternative
		 * for that has the best evaluation according to a given EvaluationFunction. What
		 * value is considered "best" is not defined in the abstract base definition but must be
		 * computed in any concrete specialization.
		 */
	
		subject selectedAlternative : Anything {
			doc
			/*
			 * The alternative that should be selected, as evaluated using the given 
			 * ObjectiveFunction.
			 */
		}
		
		in ref alternatives : Anything[1..*] {
			doc
			/*
			 * The alternatives being considered in the TradeStudy for which this TradeStudyObjective 
			 * is the objective.
			 */
		}
		
		in calc eval : EvaluationFunction {
			doc
			/*
			 * The EvaluationFunction to be used in evaluating the given alternatives.
			 */
		}
		
		attribute best : ScalarValue {
			doc
			/*
			 * Out of the evaluation results of all the given alternatives, the one that is considered 
			 * "best", in the sense that it is the value the selectedAlternative should have. This 
			 * value must be computed in any concrete specialization of TradeStudyObjective.
			 */
		}
				
		require constraint { eval(selectedAlternative) == best }
	}
	
	requirement def MinimizeObjective :> TradeStudyObjective {
		doc
		/*
		 * A MinimizeObjective is a TradeStudyObjective that requires that the 
		 * selectedAlternative have the minimum ObjectiveFunction value of all the
		 * given alternatives.
		 */
		 
		attribute :>> best = alternatives->minimize {
			doc
			/*
			 * For a MinimizeObjective, the best value is the minimum one.
			 */
		
			in x; eval(x)
		};
	}
	
	requirement def MaximizeObjective :> TradeStudyObjective {
		doc
		/*
		 * A MaximizeObjective is a TradeStudyObjective that requires that the 
		 * selectedAlternative have the maximum ObjectiveFunction value of all the
		 * given alternatives.
		 */
	
		attribute :>> best = alternatives->maximize {
			doc
			/*
			 * For a MinimizeObjective, the best value is the maximum one.
			 */
		
			in x; eval(x)
		};
	}
	
	abstract analysis def TradeStudy {
		doc
		/*
		 * A TradeStudy is an analysis case whose subject is a set of alternatives
		 * (at least one) and whose result is a selection of one of those alternatives.
		 * The alternatives are evaluated based on a given ObjectiveFunction and the
		 * selection is made such that it satisfies the objective of the TradeStudy
		 * (which must be a TradeStudyObjective).
		 */
	
		subject studyAlternatives : Anything[1..*] {
			doc
			/*
			 * The set of alternatives being considered in this TradeStudy. 
			 * 
			 * In a TradeStudy usage, bind this feature to the actual collection of
			 * alternatives to be considered.
			 */
		}
		
		abstract calc evaluationFunction : EvaluationFunction {
			doc
			/*
			 * The EvaluationFunction to be used to evaluate the alternatives.
			 * 
			 * In a TradeStudy usage, redefine this feature to provide the desired
			 * calculation (or bind it to a calculation usage that does so).
			 */
		}
		
		objective tradeStudyObjective : TradeStudyObjective {
			doc
			/*
			 * The objective of this TradeStudy.
			 * 
			 * Redefine this feature to give it a definition that is a concrete
			 * specialization of TradeStudyObjective. That can either be one of the
			 * specializations provided in this package, or a more specific user-
			 * defined one.
			 */
		
            subject :>> selectedAlternative;
			in ref :>> alternatives = studyAlternatives;
			in calc :>> eval = evaluationFunction;
		}
		
		return selectedAlternative : Anything = studyAlternatives->selectOne {in ref a {
			doc
			/*
			 * The alternative selected by this TradeStudy, which is the one that meets the
			 * requirement of the tradeStudyObjective.
			 */
		} tradeStudyObjective(selectedAlternative = a)};
	}
	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3b8b3ca9879a4b2eab2d1b272963a8a82ae8c3967a2a9b6ea4853f027762a326") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TradeStudies"))) (kind "package") (name "TradeStudies") (declared-name "TradeStudies"))
    (element (id (node (document "d0") (qualified-name "TradeStudies::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "TradeStudies"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "TradeStudies"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "TradeStudies"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "TradeStudies"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction"))) (kind "calc def") (name "EvaluationFunction") (declared-name "EvaluationFunction") (parent (node (document "d0") (qualified-name "TradeStudies"))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction"))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction"))) (authored (relationships (typing (reference "ref alternative : Anything")))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective"))) (kind "requirement def") (name "MaximizeObjective") (declared-name "MaximizeObjective") (parent (node (document "d0") (qualified-name "TradeStudies"))) (authored (membership (kind Owning)) (relationships (specializes (reference "TradeStudyObjective")))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective"))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective::best"))) (kind "attribute") (name "best") (declared-name "best") (parent (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective"))) (authored (relationships (redefinition (reference "best")))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective"))) (kind "requirement def") (name "MinimizeObjective") (declared-name "MinimizeObjective") (parent (node (document "d0") (qualified-name "TradeStudies"))) (authored (membership (kind Owning)) (relationships (specializes (reference "TradeStudyObjective")))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective"))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective::best"))) (kind "attribute") (name "best") (declared-name "best") (parent (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective"))) (authored (relationships (redefinition (reference "best")))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy"))) (kind "analysis def") (name "TradeStudy") (declared-name "TradeStudy") (parent (node (document "d0") (qualified-name "TradeStudies"))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "TradeStudies::TradeStudy"))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (kind "calc") (name "evaluationFunction") (declared-name "evaluationFunction") (parent (node (document "d0") (qualified-name "TradeStudies::TradeStudy"))) (authored (membership (kind Feature)) (relationships (typing (reference "EvaluationFunction")))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy::evaluationFunction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy::selectedAlternative"))) (kind "analysis result") (name "selectedAlternative") (declared-name "selectedAlternative") (parent (node (document "d0") (qualified-name "TradeStudies::TradeStudy"))) (authored (relationships (typing (reference "Anything")))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (kind "subject") (name "studyAlternatives") (declared-name "studyAlternatives") (parent (node (document "d0") (qualified-name "TradeStudies::TradeStudy"))) (authored (relationships (typing (reference "Anything")))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (kind "objective") (name "tradeStudyObjective") (declared-name "tradeStudyObjective") (parent (node (document "d0") (qualified-name "TradeStudies::TradeStudy"))) (authored (relationships (typing (reference "TradeStudyObjective")))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))) (kind "requirement def") (name "TradeStudyObjective") (declared-name "TradeStudyObjective") (parent (node (document "d0") (qualified-name "TradeStudies"))) (authored (membership (kind Owning)) (relationships (subject (reference "TradeStudies::TradeStudyObjective::selectedAlternative")))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (kind "attribute") (name "best") (declared-name "best") (parent (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))) (authored (relationships (typing (reference "ScalarValue")))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (kind "subject") (name "selectedAlternative") (declared-name "selectedAlternative") (parent (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))) (authored (relationships (typing (reference "Anything")))))
    (element (id (node (document "d0") (qualified-name "TradeStudies::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "TradeStudies"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ControlFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref alternative : Anything") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective"))) (kind specialization) (ordinal 0)) (authored-target "TradeStudyObjective") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective::best"))) (kind redefinition) (ordinal 0)) (authored-target "best") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective::best")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective"))) (kind specialization) (ordinal 0)) (authored-target "TradeStudyObjective") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective::best"))) (kind redefinition) (ordinal 0)) (authored-target "best") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective::best")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "EvaluationFunction") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy::selectedAlternative"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudies::Anything")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudies::Anything")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (kind featureTyping) (ordinal 0)) (authored-target "TradeStudyObjective") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "TradeStudies::TradeStudyObjective::selectedAlternative") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudies::Anything")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective"))) (target (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective::best"))) (target (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective::best"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective::best"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective"))) (target (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective::best"))) (target (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective::best"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective::best"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy"))) (target (node (document "d0") (qualified-name "TradeStudies::Anything"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (target (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy::selectedAlternative"))) (target (node (document "d0") (qualified-name "TradeStudies::Anything"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy::selectedAlternative"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (target (node (document "d0") (qualified-name "TradeStudies::Anything"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (target (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))) (target (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))) (target (node (document "d0") (qualified-name "TradeStudies::Anything"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (target (node (document "d0") (qualified-name "TradeStudies::Anything"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective::best")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective::best")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "TradeStudies::TradeStudy")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "TradeStudies::TradeStudy::selectedAlternative")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::_requireConstraint_0")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 87 16) (end 87 20)) (probe (position 87 16))
      (reference
        (source (document "d0") (qualified-name "TradeStudies::MinimizeObjective::best"))
        (kind redefinition) (ordinal 0) (authored-target "best")
        (range (start 87 16) (end 87 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TradeStudies::MinimizeObjective::best") (range (start 87 2) (end 87 158)))
        )
      )
    )
    (query (range (start 105 16) (end 105 20)) (probe (position 105 16))
      (reference
        (source (document "d0") (qualified-name "TradeStudies::MaximizeObjective::best"))
        (kind redefinition) (ordinal 0) (authored-target "best")
        (range (start 105 16) (end 105 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TradeStudies::MaximizeObjective::best") (range (start 105 2) (end 105 158)))
        )
      )
    )
    (query (range (start 7 16) (end 7 28)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "TradeStudies::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 7 16) (end 7 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 30)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "TradeStudies::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 6 16) (end 6 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 31)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "TradeStudies::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarFunctions::*")
        (range (start 8 16) (end 8 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 32)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "TradeStudies::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions::*")
        (range (start 9 16) (end 9 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 79 38) (end 79 57)) (probe (position 79 38))
      (reference
        (source (document "d0") (qualified-name "TradeStudies::MinimizeObjective"))
        (kind specialization) (ordinal 0) (authored-target "TradeStudyObjective")
        (range (start 79 38) (end 79 57))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TradeStudies::TradeStudyObjective") (range (start 34 1) (end 34 1358)))
        )
      )
    )
    (query (range (start 97 38) (end 97 57)) (probe (position 97 38))
      (reference
        (source (document "d0") (qualified-name "TradeStudies::MaximizeObjective"))
        (kind specialization) (ordinal 0) (authored-target "TradeStudyObjective")
        (range (start 97 38) (end 97 57))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TradeStudies::TradeStudyObjective") (range (start 34 1) (end 34 1358)))
        )
      )
    )
  )
)
~~~
