# META
~~~ini
description=Standard Library: Systems Library/Requirements
type=file
~~~
# SOURCE
~~~sysml
standard library package Requirements {
	doc
	/*
	 * This package defines the base types for requirements and related elements in the SysML language.
	 */

	private import Base::Anything;
	private import ScalarValues::String;
	private import ControlFunctions::allTrue;
	private import Constraints::constraintChecks;
	private import Constraints::assertedConstraintChecks;
	private import Constraints::negatedConstraintChecks;
	private import Parts::Part;
	private import Parts::parts;
	private import Actions::Action;
	private import Interfaces::Interface;
	private import Attributes::AttributeValue;
	
	private abstract constraint def RequirementConstraintCheck {
		doc
		/*
		 * RequirementConstraintCheck is the base ConstraintCheck for RequirementCheck, defining the
		 * separate assumptions and required constraints such that, if all the assumptions are true,
		 * then all the required constraints must be true.
		 */
	
		constraint assumptions[0..*] :> constraintChecks, subperformances {
			doc
			/*
			 * Assumptions that must hold for the required constraints to apply.
			 */
		}
		
		constraint constraints[0..*] :> constraintChecks, subperformances {
			doc
			/*
			 * The required constraints that are to be checked.
			 */
		}
		
		return result = allTrue(assumptions()) implies allTrue(constraints()) {
			doc
			/*
			 * If all the assumptions are true, then all the required constraints must hold.
			 */
		}
	}
	
	abstract requirement def RequirementCheck :> RequirementConstraintCheck {
		doc
		/*
		 * RequirementCheck is the most general class for requirements checking. RequirementsCheck is the base
		 * type of all requirement definitions.
		 */
	
		ref requirement :>> self: RequirementCheck;
		
		subject subj : Anything[1] {
			doc
			/*
			 * The entity that is being checked for satisfaction of the required constraints.
			 */
		}
		
		ref part actors : Part[0..*] {
			doc
			/*
			 * The Parts that fill the role of actors for this RequirementCheck.
			 * (Note: This is not itself an actor parameter, because specific actor
			 * parameters will be added for specific RequirementChecks.)
			 */
		}
		
		ref part stakeholders : Part[0..*] {
			doc
			/*
			 * The Parts that represent stakeholders interested in the concern being checked.
			 * (Note: This is not itself a stakeholder parameter, because specific stakeholder
			 * parameters will be added for specific RequirementChecks.)
			 */
		}

		/* 
		 * Note: assumptions and constraints are redefined here solely to simplify the
		 * resolution of their qualified names as library elements.
		 */
		constraint assumptions :>> RequirementConstraintCheck::assumptions;
		constraint constraints :>> RequirementConstraintCheck::constraints;
		
		abstract requirement subrequirements[0..*] :> requirementChecks, constraints {
			doc
			/*
			 * Nested requirements, which are also required constraints.
			 */
		}
		
		abstract concern concerns[0..*] :> concernChecks, subrequirements {
			doc
			/*
			 * The checks of any concerns being addressed (as required constraints).
			 */
		}
		
	}
	
	requirement def FunctionalRequirementCheck :> RequirementCheck {
		doc
		/*
		 * A functional requirement specifies an action that a system, or part of a system, must perform.
		 */
	
		subject: Action;
	}
	
	requirement def InterfaceRequirementCheck :> RequirementCheck {
		doc
		/*
		 * An interface requirement specifies an interface for connecting systems and system parts, which
		 * optionally may include item flows across the interface and/or interface constraints.
		 */
	
		subject: Interface;
	}
	
	requirement def PerformanceRequirementCheck :> RequirementCheck {
		doc
		/*
		 * A performance requirement quantitavely measures the extent to which a system, or a system part, 
		 * satisfies a required capability or condition.
		 */
	
		subject: AttributeValue;
	}
	
	requirement def PhysicalRequirementCheck :> RequirementCheck {
		doc
		/*
		 * A physical requirement specifies physical characteristics and/or physical constraints of the 
		 * system, or a system part.
		 */
	
		subject: Part;
	}
	
	requirement def DesignConstraintCheck :> RequirementCheck {
		doc
		/*
		 * A design constraint specifies a constraint on the implementation of the system or system part, 
		 * such as the system must use a commercial off the shelf component.
		 */
	
		subject: Part;
	}
	
	concern def ConcernCheck :> RequirementCheck {
		doc
		/*
		 * ConcernCheck is the most general class for concern checking. ConcernCheck is the base type of 
		 * all ConcernDefinitions.
		 */
	
		ref concern :>> self: ConcernCheck;
		
	}
	
	abstract requirement requirementChecks: RequirementCheck[0..*] nonunique :> constraintChecks {
		doc
		/*
		 * requirementChecks is the base feature of all requirement usages.
		 */
	}
	
	abstract requirement satisfiedRequirementChecks :> requirementChecks, assertedConstraintChecks {
		doc
		/*
		 * satisfiedRequirementChecks is the subset of requirementChecks for Requirements asserted to be satisfied.
		 */
	}

	abstract requirement notSatisfiedRequirementChecks: RequirementCheck[0..*] :> requirementChecks, negatedConstraintChecks {
		doc
		/*
		 * notSatisfiedRequirementChecks is the subset of requirementChecks for Requirements asserted to be not satisfied.
		 */
	}
	
	abstract concern concernChecks: ConcernCheck[0..*] nonunique :> requirementChecks {
		doc
		/*
		 * concernChecks is the base feature of all ConcernUsages.
		 */
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "requirements.md"
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
        (range (start 7 16) (end 7 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 42))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "be63cbdf3e3950de0494607dce1518b87f9600c3bf627700015d0e4151a852b4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Requirements"))) (kind "package") (name "Requirements") (declared-name "Requirements"))
    (element (id (node (document "d0") (qualified-name "Requirements::Action"))) (kind "import") (name "Action") (declared-name "Action") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::Action") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirements::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirements::AttributeValue"))) (kind "import") (name "AttributeValue") (declared-name "AttributeValue") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Attributes::AttributeValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirements::ConcernCheck"))) (kind "concern def") (name "ConcernCheck") (declared-name "ConcernCheck") (parent (node (document "d0") (qualified-name "Requirements"))))
    (element (id (node (document "d0") (qualified-name "Requirements::ConcernCheck::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::ConcernCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (kind "requirement def") (name "DesignConstraintCheck") (declared-name "DesignConstraintCheck") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementCheck")) (subject (reference "Requirements::DesignConstraintCheck::")))))
    (element (id (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::"))) (kind "subject") (name "") (parent (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (authored (relationships (typing (reference "Part")))))
    (element (id (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind "requirement def") (name "FunctionalRequirementCheck") (declared-name "FunctionalRequirementCheck") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementCheck")) (subject (reference "Requirements::FunctionalRequirementCheck::")))))
    (element (id (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::"))) (kind "subject") (name "") (parent (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (authored (relationships (typing (reference "Action")))))
    (element (id (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::Interface"))) (kind "import") (name "Interface") (declared-name "Interface") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Interfaces::Interface") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind "requirement def") (name "InterfaceRequirementCheck") (declared-name "InterfaceRequirementCheck") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementCheck")) (subject (reference "Requirements::InterfaceRequirementCheck::")))))
    (element (id (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::"))) (kind "subject") (name "") (parent (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (authored (relationships (typing (reference "Interface")))))
    (element (id (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::Part"))) (kind "import") (name "Part") (declared-name "Part") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::Part") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind "requirement def") (name "PerformanceRequirementCheck") (declared-name "PerformanceRequirementCheck") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementCheck")) (subject (reference "Requirements::PerformanceRequirementCheck::")))))
    (element (id (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::"))) (kind "subject") (name "") (parent (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (authored (relationships (typing (reference "AttributeValue")))))
    (element (id (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind "requirement def") (name "PhysicalRequirementCheck") (declared-name "PhysicalRequirementCheck") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementCheck")) (subject (reference "Requirements::PhysicalRequirementCheck::")))))
    (element (id (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::"))) (kind "subject") (name "") (parent (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (authored (relationships (typing (reference "Part")))))
    (element (id (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (kind "requirement def") (name "RequirementCheck") (declared-name "RequirementCheck") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementConstraintCheck")) (subject (reference "Requirements::RequirementCheck::subj")))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementCheck::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::RequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (kind "subject") (name "subj") (declared-name "subj") (parent (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (authored (relationships (typing (reference "Anything")))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind "requirement") (name "subrequirements") (declared-name "subrequirements") (parent (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "requirementChecks")) (subsetting (reference "constraints")))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck"))) (kind "constraint def") (name "RequirementConstraintCheck") (declared-name "RequirementConstraintCheck") (parent (node (document "d0") (qualified-name "Requirements"))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirements::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements"))))
    (element (id (node (document "d0") (qualified-name "Requirements::allTrue"))) (kind "import") (name "allTrue") (declared-name "allTrue") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::allTrue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirements::assertedConstraintChecks"))) (kind "import") (name "assertedConstraintChecks") (declared-name "assertedConstraintChecks") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Constraints::assertedConstraintChecks") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirements::assumptions"))) (kind "constraint") (name "assumptions") (declared-name "assumptions") (parent (node (document "d0") (qualified-name "Requirements::RequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::concernChecks"))) (kind "concern") (name "concernChecks") (declared-name "concernChecks") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConcernCheck")))))
    (element (id (node (document "d0") (qualified-name "Requirements::concernChecks::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::concernChecks"))))
    (element (id (node (document "d0") (qualified-name "Requirements::constraintChecks"))) (kind "import") (name "constraintChecks") (declared-name "constraintChecks") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Constraints::constraintChecks") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirements::constraints"))) (kind "constraint") (name "constraints") (declared-name "constraints") (parent (node (document "d0") (qualified-name "Requirements::RequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::negatedConstraintChecks"))) (kind "import") (name "negatedConstraintChecks") (declared-name "negatedConstraintChecks") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Constraints::negatedConstraintChecks") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind "requirement") (name "notSatisfiedRequirementChecks") (declared-name "notSatisfiedRequirementChecks") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "RequirementCheck")) (subsetting (reference "requirementChecks")) (subsetting (reference "negatedConstraintChecks")))))
    (element (id (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))))
    (element (id (node (document "d0") (qualified-name "Requirements::parts"))) (kind "import") (name "parts") (declared-name "parts") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::parts") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (kind "requirement") (name "requirementChecks") (declared-name "requirementChecks") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "RequirementCheck")) (subsetting (reference "constraintChecks")))))
    (element (id (node (document "d0") (qualified-name "Requirements::requirementChecks::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::requirementChecks"))))
    (element (id (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind "requirement") (name "satisfiedRequirementChecks") (declared-name "satisfiedRequirementChecks") (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "requirementChecks")) (subsetting (reference "assertedConstraintChecks")))))
    (element (id (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Requirements::Action"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::Action") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::AttributeValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Attributes::AttributeValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementCheck") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirements::DesignConstraintCheck::") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementCheck") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirements::FunctionalRequirementCheck::") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::Interface"))) (kind membershipImport) (ordinal 0)) (authored-target "Interfaces::Interface") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementCheck") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirements::InterfaceRequirementCheck::") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::"))) (kind featureTyping) (ordinal 0)) (authored-target "Interface") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::Interface")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::Part"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::Part") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementCheck") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirements::PerformanceRequirementCheck::") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::"))) (kind featureTyping) (ordinal 0)) (authored-target "AttributeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::AttributeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementCheck") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirements::PhysicalRequirementCheck::") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementConstraintCheck") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirements::RequirementCheck::subj") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::Anything")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 0)) (authored-target "requirementChecks") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::requirementChecks")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 1)) (authored-target "constraints") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::constraints")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::allTrue"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::allTrue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::assertedConstraintChecks"))) (kind membershipImport) (ordinal 0)) (authored-target "Constraints::assertedConstraintChecks") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::concernChecks"))) (kind featureTyping) (ordinal 0)) (authored-target "ConcernCheck") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::ConcernCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::constraintChecks"))) (kind membershipImport) (ordinal 0)) (authored-target "Constraints::constraintChecks") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::negatedConstraintChecks"))) (kind membershipImport) (ordinal 0)) (authored-target "Constraints::negatedConstraintChecks") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementCheck") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 0)) (authored-target "requirementChecks") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::requirementChecks")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 1)) (authored-target "negatedConstraintChecks") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::negatedConstraintChecks")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::parts"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::parts") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementCheck") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (kind subsetting) (ordinal 0)) (authored-target "constraintChecks") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::constraintChecks")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 0)) (authored-target "requirementChecks") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::requirementChecks")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 1)) (authored-target "assertedConstraintChecks") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::assertedConstraintChecks")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (target (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (target (node (document "d0") (qualified-name "Requirements::Part"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::"))) (target (node (document "d0") (qualified-name "Requirements::Part"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::Action"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::"))) (target (node (document "d0") (qualified-name "Requirements::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::Interface"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::"))) (target (node (document "d0") (qualified-name "Requirements::Interface"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::AttributeValue"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::"))) (target (node (document "d0") (qualified-name "Requirements::AttributeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::Part"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::"))) (target (node (document "d0") (qualified-name "Requirements::Part"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::Anything"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (target (node (document "d0") (qualified-name "Requirements::Anything"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (target (node (document "d0") (qualified-name "Requirements::constraints"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (target (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::concernChecks"))) (target (node (document "d0") (qualified-name "Requirements::ConcernCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::concernChecks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::negatedConstraintChecks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::constraintChecks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::assertedConstraintChecks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 12 16) (end 12 27)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "Requirements::Part"))
        (kind membershipImport) (ordinal 0) (authored-target "Parts::Part")
        (range (start 12 16) (end 12 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 89 67) (end 89 78)) (probe (position 89 67))
      (reference
        (source (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))
        (kind subsetting) (ordinal 1) (authored-target "constraints")
        (range (start 89 67) (end 89 78))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::constraints") (range (start 87 2) (end 87 69)))
        )
      )
    )
    (query (range (start 13 16) (end 13 28)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "Requirements::parts"))
        (kind membershipImport) (ordinal 0) (authored-target "Parts::parts")
        (range (start 13 16) (end 13 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 30)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "Requirements::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 6 16) (end 6 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 31)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "Requirements::Action"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
        (range (start 14 16) (end 14 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 105 47) (end 105 63)) (probe (position 105 47))
      (reference
        (source (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))
        (kind specialization) (ordinal 0) (authored-target "RequirementCheck")
        (range (start 105 47) (end 105 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::RequirementCheck") (range (start 48 1) (end 48 1648)))
        )
      )
    )
    (query (range (start 114 46) (end 114 62)) (probe (position 114 46))
      (reference
        (source (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))
        (kind specialization) (ordinal 0) (authored-target "RequirementCheck")
        (range (start 114 46) (end 114 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::RequirementCheck") (range (start 48 1) (end 48 1648)))
        )
      )
    )
    (query (range (start 124 48) (end 124 64)) (probe (position 124 48))
      (reference
        (source (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))
        (kind specialization) (ordinal 0) (authored-target "RequirementCheck")
        (range (start 124 48) (end 124 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::RequirementCheck") (range (start 48 1) (end 48 1648)))
        )
      )
    )
    (query (range (start 134 45) (end 134 61)) (probe (position 134 45))
      (reference
        (source (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))
        (kind specialization) (ordinal 0) (authored-target "RequirementCheck")
        (range (start 134 45) (end 134 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::RequirementCheck") (range (start 48 1) (end 48 1648)))
        )
      )
    )
    (query (range (start 144 42) (end 144 58)) (probe (position 144 42))
      (reference
        (source (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))
        (kind specialization) (ordinal 0) (authored-target "RequirementCheck")
        (range (start 144 42) (end 144 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::RequirementCheck") (range (start 48 1) (end 48 1648)))
        )
      )
    )
    (query (range (start 165 77) (end 165 93)) (probe (position 165 77))
      (reference
        (source (document "d0") (qualified-name "Requirements::requirementChecks"))
        (kind subsetting) (ordinal 0) (authored-target "constraintChecks")
        (range (start 165 77) (end 165 93))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::constraintChecks") (range (start 9 1) (end 9 46)))
        )
      )
    )
    (query (range (start 89 48) (end 89 65)) (probe (position 89 48))
      (reference
        (source (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))
        (kind subsetting) (ordinal 0) (authored-target "requirementChecks")
        (range (start 89 48) (end 89 65))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::requirementChecks") (range (start 165 1) (end 165 185)))
        )
      )
    )
    (query (range (start 172 52) (end 172 69)) (probe (position 172 52))
      (reference
        (source (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))
        (kind subsetting) (ordinal 0) (authored-target "requirementChecks")
        (range (start 172 52) (end 172 69))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::requirementChecks") (range (start 165 1) (end 165 185)))
        )
      )
    )
    (query (range (start 179 79) (end 179 96)) (probe (position 179 79))
      (reference
        (source (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))
        (kind subsetting) (ordinal 0) (authored-target "requirementChecks")
        (range (start 179 79) (end 179 96))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::requirementChecks") (range (start 165 1) (end 165 185)))
        )
      )
    )
    (query (range (start 7 16) (end 7 36)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Requirements::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 7 16) (end 7 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 37)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "Requirements::Interface"))
        (kind membershipImport) (ordinal 0) (authored-target "Interfaces::Interface")
        (range (start 15 16) (end 15 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 179 98) (end 179 121)) (probe (position 179 98))
      (reference
        (source (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))
        (kind subsetting) (ordinal 1) (authored-target "negatedConstraintChecks")
        (range (start 179 98) (end 179 121))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::negatedConstraintChecks") (range (start 11 1) (end 11 53)))
        )
      )
    )
    (query (range (start 172 71) (end 172 95)) (probe (position 172 71))
      (reference
        (source (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))
        (kind subsetting) (ordinal 1) (authored-target "assertedConstraintChecks")
        (range (start 172 71) (end 172 95))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::assertedConstraintChecks") (range (start 10 1) (end 10 54)))
        )
      )
    )
    (query (range (start 8 16) (end 8 41)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Requirements::allTrue"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::allTrue")
        (range (start 8 16) (end 8 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 42)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "Requirements::AttributeValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Attributes::AttributeValue")
        (range (start 16 16) (end 16 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 48 46) (end 48 72)) (probe (position 48 46))
      (reference
        (source (document "d0") (qualified-name "Requirements::RequirementCheck"))
        (kind specialization) (ordinal 0) (authored-target "RequirementConstraintCheck")
        (range (start 48 46) (end 48 72))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirements::RequirementConstraintCheck") (range (start 18 1) (end 18 829)))
        )
      )
    )
    (query (range (start 9 16) (end 9 45)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Requirements::constraintChecks"))
        (kind membershipImport) (ordinal 0) (authored-target "Constraints::constraintChecks")
        (range (start 9 16) (end 9 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 52)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "Requirements::negatedConstraintChecks"))
        (kind membershipImport) (ordinal 0) (authored-target "Constraints::negatedConstraintChecks")
        (range (start 11 16) (end 11 52))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 53)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Requirements::assertedConstraintChecks"))
        (kind membershipImport) (ordinal 0) (authored-target "Constraints::assertedConstraintChecks")
        (range (start 10 16) (end 10 53))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
